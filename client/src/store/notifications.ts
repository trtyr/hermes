import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { notification as antNotification } from 'ant-design-vue';
import { useEventStore } from './events';
import type { BackendEvent } from './events';

export interface Notification {
  id: string;
  type: 'success' | 'warning' | 'info' | 'error';
  title: string;
  message: string;
  timestamp: number;
  read: boolean;
  /** 路由跳转，点击通知时导航 */
  route?: string;
}

const MAX_NOTIFICATIONS = 100;

export const useNotificationStore = defineStore('notifications', () => {
  const items = ref<Notification[]>([]);
  const unreadCount = computed(() => items.value.filter(n => !n.read).length);
  let initialized = false;

  function add(notif: Omit<Notification, 'id' | 'timestamp' | 'read'>, silent = false) {
    items.value.unshift({
      ...notif,
      id: `${Date.now()}-${Math.random().toString(36).slice(2, 6)}`,
      timestamp: Date.now(),
      read: false,
    });
    if (items.value.length > MAX_NOTIFICATIONS) {
      items.value = items.value.slice(0, MAX_NOTIFICATIONS);
    }

    if (!silent) {
      antNotification.open({
        message: notif.title,
        description: notif.message,
        duration: 4,
        placement: 'topRight',
      });
    }
  }

  function markAllRead() {
    items.value.forEach(n => n.read = true);
  }

  function clearAll() {
    items.value = [];
  }

  function markRead(id: string) {
    const n = items.value.find(item => item.id === id);
    if (n) n.read = true;
  }

  function init() {
    if (initialized) return;
    initialized = true;

    const eventStore = useEventStore();
    eventStore.subscribe((event: BackendEvent) => {
      switch (event.type) {
        case 'agent_registered':
          add({
            type: 'success',
            title: 'Agent 上线',
            message: `${event.agent.hostname || event.agent.agent_id} (${event.agent.internal_ip || event.agent.peer_addr})`,
            route: '/agent',
          });
          break;
        case 'agent_disconnected': {
          const displayName = event.agent_id ? eventStore.getAgentDisplayName(event.agent_id) : 'unknown';
          add({
            type: 'warning',
            title: 'Agent 离线',
            message: displayName,
            route: '/agent',
          });
          break;
        }
        case 'task_result': {
          // Generate meaningful notification based on command type
          let taskLabel = `Task ${event.task_id}`;
          const cmd = event.command || '';
          if (cmd === 'download') {
            taskLabel = '文件下载完成';
          } else if (cmd === 'upload') {
            taskLabel = '文件上传完成';
          } else if (cmd === 'browse') {
            taskLabel = '目录浏览完成';
          } else if (cmd) {
            // Generic command execution
            const preview = cmd.length > 40 ? cmd.slice(0, 40) + '...' : cmd;
            taskLabel = `命令执行: ${preview}`;
          }
          add({
            type: event.success ? 'success' : 'error',
            title: event.success ? '任务完成' : '任务失败',
            message: taskLabel,
            route: '/agent',
          });
          break;
        }
        case 'agent_build_completed':
          add({
            type: event.build.status === 'succeeded' ? 'success' : 'error',
            title: '载荷构建完成',
            message: `Build #${event.build.build_id} — ${event.build.status}`,
            route: '/payload',
          });
          break;
        case 'agent_deleted':
          add({
            type: 'info',
            title: 'Agent 已删除',
            message: event.agent_id,
          }, true);
          break;
      }
    });
  }

  return { items, unreadCount, add, markAllRead, clearAll, markRead, init };
});
