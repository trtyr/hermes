<template>
  <div>
    <div 
      v-if="eventStore.isConnected" 
      class="flex items-center space-x-2 bg-green-50 text-green-600 px-3 py-1.5 rounded-full text-sm border border-green-200 transition-colors"
    >
      <div class="w-2 h-2 rounded-full bg-green-500 animate-pulse"></div>
      <span>已连接: {{ connectionStore.activeProfile?.connection_name || connectionStore.activeProfile?.server_url }}</span>
    </div>
    
    <div 
      v-else 
      class="flex items-center space-x-2 bg-red-50 text-red-600 px-3 py-1.5 rounded-full text-sm border border-red-200 transition-colors"
    >
      <div class="w-2 h-2 rounded-full bg-red-500"></div>
      <span>已断开/重连中: {{ eventStore.lastError || '后端失联' }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useConnectionStore } from '@/store/connection';
import { useEventStore } from '@/store/events';

const connectionStore = useConnectionStore();
const eventStore = useEventStore();
</script>
