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
        case 'agent_disconnected':
          add({
            type: 'warning',
            title: 'Agent 离线',
            message: event.agent_id || 'unknown',
            route: '/agent',
          });
          break;
        case 'task_result':
          add({
            type: event.success ? 'success' : 'error',
            title: event.success ? '任务完成' : '任务失败',
            message: `Task ${event.task_id}`,
            route: '/agent',
          });
          break;
        case 'agent_build_completed':
          add({
            type: event.status === 'completed' ? 'success' : 'error',
            title: '载荷构建完成',
            message: `Build #${event.build_id} — ${event.status}`,
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
