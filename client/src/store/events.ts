import { defineStore } from 'pinia';
import { ref, watch } from 'vue';
import { useConnectionStore } from './connection';
import type { Agent } from '@/api/agent';

export type BackendEvent = 
  | { type: 'snapshot'; agents: Agent[] }
  | { type: 'agent_connected'; session_id: number; peer_addr: string; connected_at: number }
  | { type: 'agent_registered'; agent: Agent }
  | { type: 'agent_heartbeat'; session_id: number; agent_id: string; last_seen: number }
  | { type: 'agent_updated'; agent: Agent }
  | { type: 'agent_disconnected'; session_id: number; agent_id: string }
  | { type: 'agent_disabled'; agent_id: string }
  | { type: 'agent_enabled'; agent_id: string }
  | { type: 'agent_deleted'; agent_id: string }
  | { type: 'task_dispatched'; [key: string]: any }
  | { type: 'task_result'; [key: string]: any }
  | { type: 'task_updated'; [key: string]: any };

export const useEventStore = defineStore('events', () => {
  const connectionStore = useConnectionStore();
  
  const socket = ref<WebSocket | null>(null);
  const isConnected = ref(false);
  const lastError = ref<string | null>(null);
  const reconnectTimer = ref<any>(null);
  const retryCount = ref(0);
  const manualDisconnect = ref(false);

  const subscribers = ref<Set<(event: BackendEvent) => void>>(new Set());

  function subscribe(callback: (event: BackendEvent) => void) {
    subscribers.value.add(callback);
    return () => subscribers.value.delete(callback);
  }

  function notifySubscribers(event: BackendEvent) {
    subscribers.value.forEach(cb => cb(event));
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

      console.log('[EventStore] Connecting to', wsUrl);
      const ws = new WebSocket(wsUrl);
      socket.value = ws;

      ws.onopen = () => {
        console.log('[EventStore] WebSocket connected');
        isConnected.value = true;
        lastError.value = null;
        retryCount.value = 0;
      };

      ws.onmessage = (msg) => {
        try {
          const payload = JSON.parse(msg.data) as BackendEvent;
          notifySubscribers(payload);
        } catch (e) {
          console.error('[EventStore] Failed to parse message', e);
        }
      };

      ws.onerror = (e) => {
        console.error('[EventStore] WebSocket error', e);
        lastError.value = 'Connection error';
      };

      ws.onclose = (e) => {
        console.log('[EventStore] WebSocket closed', e.code, e.reason);
        isConnected.value = false;
        socket.value = null;
        
        if (!manualDisconnect.value && connectionStore.activeProfile) {
          scheduleReconnect();
        }
      };

    } catch (e: any) {
      console.error('[EventStore] Connection setup failed', e);
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

    const delay = Math.min(1000 * Math.pow(2, retryCount.value), 30000);
    console.log(`[EventStore] Reconnecting in ${delay}ms...`);
    
    reconnectTimer.value = setTimeout(() => {
      reconnectTimer.value = null;
      retryCount.value++;
      connect();
    }, delay);
  }

  // Auto connect/disconnect when active profile changes
  watch(() => connectionStore.activeProfileId, (newId) => {
    if (newId) {
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
    subscribe
  };
});
