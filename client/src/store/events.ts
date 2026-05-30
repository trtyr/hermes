import { defineStore } from 'pinia';
import { ref, watch } from 'vue';
import { useConnectionStore } from './connection';
import type { Agent } from '@/api/agent';

// After this many consecutive reconnect failures, assume the session is gone.
const MAX_RETRIES_BEFORE_LOGOUT = 6;

export type BackendEvent =
  | { type: 'snapshot'; agents: Agent[] }
  | { type: 'agent_connected'; session_id: number; peer_addr: string; connected_at: number }
  | { type: 'agent_registered'; agent: Agent }
  | { type: 'agent_heartbeat'; session_id: number; agent_id: string; last_seen: number }
  | { type: 'agent_updated'; agent: Agent }
  | { type: 'agent_disconnected'; session_id: number; agent_id: string | null }
  | { type: 'agent_disabled'; agent_id: string }
  | { type: 'agent_enabled'; agent_id: string }
  | { type: 'agent_deleted'; agent_id: string }
  | { type: 'task_dispatched'; task_id: string; target_agent_id: string }
  | { type: 'task_result'; task_id: string; agent_id: string | null; command: string; success: boolean; output: string }
  | { type: 'task_updated'; task_id: string }
  | { type: 'agent_build_created'; build: { build_id: number; status: string; [key: string]: any } }
  | { type: 'agent_build_completed'; build: { build_id: number; status: string; [key: string]: any } }
  | { type: 'agent_build_deleted'; build_id: number };

export const useEventStore = defineStore('events', () => {
  const connectionStore = useConnectionStore();
  
  const socket = ref<WebSocket | null>(null);
  const isConnected = ref(false);
  const lastError = ref<string | null>(null);
  const reconnectTimer = ref<any>(null);
  const retryCount = ref(0);
  const manualDisconnect = ref(false);

  const subscribers = ref<Set<(event: BackendEvent) => void>>(new Set());

  /** agent_id → display name (hostname or agent_id) lookup for notifications */
  const agentDisplayNames = ref<Map<string, string>>(new Map());

  /** task_id → { fileName } for pending file downloads */
  const pendingDownloads = ref<Map<string, { fileName: string }>>(new Map());

  function getAgentDisplayName(agentId: string): string {
    return agentDisplayNames.value.get(agentId) || agentId;
  }

  /** Register a pending download so that when the task_result arrives, we can trigger a browser download. */
  function registerDownload(taskId: string, fileName: string) {
    pendingDownloads.value.set(taskId, { fileName });
  }

  function subscribe(callback: (event: BackendEvent) => void) {
    subscribers.value.add(callback);
    return () => subscribers.value.delete(callback);
  }

  function notifySubscribers(event: BackendEvent) {
    subscribers.value.forEach(cb => cb(event));
  }

  /**
   * Clear the session and navigate to /login.
   * Lazy-imports router to avoid circular dependency at module init time.
   */
  async function forceLogout() {
    connectionStore.logout();
    const { router } = await import('@/router');
    router.push('/login');
  }

  function connect() {
    manualDisconnect.value = false;
    // Clean up existing
    if (socket.value) {
      socket.value.close();
      socket.value = null;
    }
    if (reconnectTimer.value) {
      clearTimeout(reconnectTimer.value);
      reconnectTimer.value = null;
    }

    const profile = connectionStore.activeProfile;
    if (!profile) return;

    try {
      const httpUrl = new URL(profile.server_url);
      const protocol = httpUrl.protocol === 'https:' ? 'wss:' : 'ws:';
      const wsUrl = `${protocol}//${httpUrl.host}/events/ws?api_token=${profile.api_token}`;

      const ws = new WebSocket(wsUrl);
      socket.value = ws;

      ws.onopen = () => {
        isConnected.value = true;
        lastError.value = null;
        retryCount.value = 0;
      };

      ws.onmessage = (msg) => {
        try {
          const payload = JSON.parse(msg.data) as BackendEvent;
          // Update agent display name map from snapshot and registration events
          if (payload.type === 'snapshot') {
            for (const agent of payload.agents) {
              agentDisplayNames.value.set(agent.agent_id, agent.hostname || agent.agent_id);
            }
          } else if (payload.type === 'agent_registered' || payload.type === 'agent_updated') {
            agentDisplayNames.value.set(payload.agent.agent_id, payload.agent.hostname || payload.agent.agent_id);
          } else if (payload.type === 'agent_deleted') {
            // Keep display name in map so notifications can still resolve it
          } else if (payload.type === 'task_result') {
            // Handle file download: decode base64 and trigger browser download
            if (payload.command === 'download' && payload.success) {
              const pending = pendingDownloads.value.get(payload.task_id);
              if (pending) {
                pendingDownloads.value.delete(payload.task_id);
                try {
                  const binaryStr = atob(payload.output);
                  const bytes = new Uint8Array(binaryStr.length);
                  for (let i = 0; i < binaryStr.length; i++) {
                    bytes[i] = binaryStr.charCodeAt(i);
                  }
                  const blob = new Blob([bytes]);
                  const url = URL.createObjectURL(blob);
                  const a = document.createElement('a');
                  a.href = url;
                  a.download = pending.fileName;
                  a.click();
                  URL.revokeObjectURL(url);
                } catch {
                  // base64 decode or download failed — notification will still show
                }
              }
            }
          }
          notifySubscribers(payload);
        } catch {
        }
      };

      ws.onerror = () => {
        lastError.value = 'Connection error';
      };

      ws.onclose = (e) => {
        isConnected.value = false;
        socket.value = null;
        
        // Close code 1008 = policy violation (often auth failure)
        // Close code 4001+ = custom server codes that may indicate auth issues
        if (e.code === 1008 || e.code === 4001) {
          forceLogout();
          return;
        }

        if (!manualDisconnect.value && connectionStore.activeProfile) {
          scheduleReconnect();
        }
      };

    } catch (e: any) {
      lastError.value = e.message;
      scheduleReconnect();
    }
  }

  function disconnect() {
    manualDisconnect.value = true;
    if (reconnectTimer.value) {
      clearTimeout(reconnectTimer.value);
      reconnectTimer.value = null;
    }
    if (socket.value) {
      socket.value.close();
      socket.value = null;
    }
    isConnected.value = false;
  }

  function scheduleReconnect() {
    if (reconnectTimer.value) return;

    // If we've retried too many times, force logout
    if (retryCount.value >= MAX_RETRIES_BEFORE_LOGOUT) {
      forceLogout();
      return;
    }

    const delay = Math.min(1000 * Math.pow(2, retryCount.value), 30000);
    
    reconnectTimer.value = setTimeout(() => {
      reconnectTimer.value = null;
      retryCount.value++;
      connect();
    }, delay);
  }

  // Auto connect/disconnect when active profile changes
  watch(() => connectionStore.activeProfileId, (newId) => {
    if (newId) {
      // Reset retry count when a new profile is activated (fresh login)
      retryCount.value = 0;
      connect();
    } else {
      disconnect();
    }
  }, { immediate: true });

  return {
    isConnected,
    lastError,
    connect,
    disconnect,
    subscribe,
    getAgentDisplayName,
    registerDownload,
  };
});
