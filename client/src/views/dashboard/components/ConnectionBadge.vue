<template>
  <div>
    <div 
      v-if="eventStore.isConnected" 
      class="flex items-center space-x-2 bg-green-50 dark:bg-green-900/20 text-green-600 dark:text-green-400 px-3 py-1.5 rounded-full text-sm border border-green-200 dark:border-green-800 transition-colors"
    >
      <div class="w-2 h-2 rounded-full bg-green-500 animate-pulse"></div>
      <span>已连接: {{ connectionStore.activeProfile?.connection_name || connectionStore.activeProfile?.server_url }}</span>
      <a-button type="link" size="small" class="p-0 h-auto ml-2" @click="emit('manage')">管理连接</a-button>
    </div>
    
    <div 
      v-else 
      class="flex items-center space-x-2 bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400 px-3 py-1.5 rounded-full text-sm border border-red-200 dark:border-red-800 transition-colors"
    >
      <div class="w-2 h-2 rounded-full bg-red-500"></div>
      <span>已断开/重连中: {{ eventStore.lastError || '后端失联' }}</span>
      <a-button type="link" size="small" class="p-0 h-auto ml-2 text-red-600 dark:text-red-400 font-semibold" @click="emit('manage')">检查设置</a-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useConnectionStore } from '@/store/connection';
import { useEventStore } from '@/store/events';

const connectionStore = useConnectionStore();
const eventStore = useEventStore();

const emit = defineEmits(['manage']);
</script>
