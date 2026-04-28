<template>
  <a-popover
    trigger="click"
    placement="bottomRight"
    :overlayStyle="{ padding: 0 }"
    v-model:open="visible"
  >
    <template #content>
      <div class="w-80 max-h-96 flex flex-col">
        <!-- 头部：标题 + 操作按钮 -->
        <div class="flex items-center justify-between px-3 py-2 border-b border-slate-100">
          <span class="font-medium text-sm text-slate-700">通知</span>
          <div class="flex gap-2 text-xs text-slate-400">
            <a class="hover:text-slate-600 cursor-pointer" @click="markAllRead">全部已读</a>
            <a class="hover:text-slate-600 cursor-pointer" @click="clearAll">清空</a>
          </div>
        </div>
        <!-- 通知列表 -->
        <div class="overflow-y-auto flex-1">
          <div v-if="notificationStore.items.length === 0" class="py-8 text-center text-slate-400 text-sm">
            暂无通知
          </div>
          <div
            v-for="item in notificationStore.items"
            :key="item.id"
            class="px-3 py-2.5 border-b border-slate-50 cursor-pointer hover:bg-slate-50 flex items-start gap-2"
            :class="{ 'bg-blue-50/40': !item.read }"
            @click="handleClick(item)"
          >
            <!-- 类型图标 -->
            <span class="mt-1">
              <span v-if="item.type === 'success'" class="block size-2 rounded-full bg-green-500"></span>
              <span v-else-if="item.type === 'warning'" class="block size-2 rounded-full bg-orange-400"></span>
              <span v-else-if="item.type === 'error'" class="block size-2 rounded-full bg-red-500"></span>
              <span v-else class="block size-2 rounded-full bg-blue-400"></span>
            </span>
            <div class="flex-1 min-w-0">
              <div class="text-sm font-medium text-slate-800">{{ item.title }}</div>
              <div class="text-xs text-slate-500 truncate">{{ item.message }}</div>
              <div class="text-xs text-slate-300 mt-0.5">{{ formatTime(item.timestamp) }}</div>
            </div>
          </div>
        </div>
      </div>
    </template>
    <slot />
  </a-popover>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { useNotificationStore } from '@/store/notifications';
import type { Notification } from '@/store/notifications';

const router = useRouter();
const notificationStore = useNotificationStore();
const visible = ref(false);

function formatTime(timestamp: number): string {
  const now = Date.now();
  const diff = now - timestamp;
  const seconds = Math.floor(diff / 1000);
  const minutes = Math.floor(seconds / 60);
  const hours = Math.floor(minutes / 60);
  const days = Math.floor(hours / 24);

  if (seconds < 60) return '刚刚';
  if (minutes < 60) return `${minutes} 分钟前`;
  if (hours < 24) return `${hours} 小时前`;
  return `${days} 天前`;
}

function handleClick(item: Notification) {
  notificationStore.markRead(item.id);
  if (item.route) {
    router.push(item.route);
    visible.value = false;
  }
}

function markAllRead() {
  notificationStore.markAllRead();
}

function clearAll() {
  notificationStore.clearAll();
}
</script>
