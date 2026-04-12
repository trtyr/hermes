<template>
  <div class="h-full w-full p-4 relative">
    <!-- Unconnected State -->
    <UnconnectedState 
      v-if="!connectionStore.activeProfile" 
      @connect="showConnectionModal = true" 
    />

    <!-- Connected State -->
    <div v-else class="h-full flex flex-col">
      <!-- Header with Status -->
      <div class="flex justify-between items-center mb-6">
        <h2 class="text-2xl font-semibold text-slate-800 dark:text-slate-100">控制台总览</h2>
        <ConnectionBadge @manage="showConnectionModal = true" />
      </div>

      <!-- Loading / Error Spinners -->
      <div v-if="loading" class="flex-1 flex flex-col items-center justify-center">
        <a-spin size="large" />
        <div class="mt-4 text-slate-400">正在加载统计数据...</div>
      </div>
      
      <div v-else-if="error" class="flex-1 flex flex-col items-center justify-center text-center">
        <div class="w-16 h-16 bg-red-50 text-red-500 rounded-full flex items-center justify-center mb-4">
          <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"></path>
          </svg>
        </div>
        <h2 class="text-xl font-semibold text-slate-800 dark:text-slate-100 mb-2">获取统计数据失败</h2>
        <p class="text-slate-500 dark:text-slate-400 mb-4 max-w-md">{{ error }}</p>
        <a-button @click="loadStats">重试</a-button>
      </div>

      <!-- Data Dashboard -->
      <div v-else-if="stats" class="space-y-6">
        <TopStatsGrid :stats="stats" />
        
        <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
          <ServerInfoCard :stats="stats" />
          <AgentsDistCard :stats="stats" />
          <ListenersDistCard :stats="stats" />
        </div>
      </div>
    </div>

    <!-- Modals -->
    <ConnectionModal v-model:visible="showConnectionModal" />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue';
import { message } from 'ant-design-vue';

// Subcomponents
import UnconnectedState from './components/UnconnectedState.vue';
import ConnectionBadge from './components/ConnectionBadge.vue';
import ConnectionModal from './components/ConnectionModal.vue';
import TopStatsGrid from './components/TopStatsGrid.vue';
import ServerInfoCard from './components/ServerInfoCard.vue';
import AgentsDistCard from './components/AgentsDistCard.vue';
import ListenersDistCard from './components/ListenersDistCard.vue';

// State and Networking
import { useConnectionStore } from '@/store/connection';
import { useEventStore } from '@/store/events';
import { fetchDashboardStats } from '@/api/dashboard';
import type { DashboardStats } from '@/api/dashboard';

const connectionStore = useConnectionStore();
const eventStore = useEventStore();

const showConnectionModal = ref(false);
const stats = ref<DashboardStats | null>(null);
const loading = ref(false);
const error = ref('');

async function loadStats() {
  if (!connectionStore.activeProfile) return;
  
  loading.value = true;
  error.value = '';
  
  try {
    stats.value = await fetchDashboardStats();
  } catch (err: any) {
    console.error('Failed to load dashboard stats:', err);
    error.value = err.message || '网络请求失败，请检查后端是否正常运行。';
    if (err.message && err.message.includes('Token')) {
      message.error(error.value);
      showConnectionModal.value = true;
    }
  } finally {
    loading.value = false;
  }
}

watch(() => connectionStore.activeProfileId, (newId) => {
  if (newId) {
    loadStats();
  } else {
    stats.value = null;
  }
});

onMounted(() => {
  if (connectionStore.activeProfile) {
    loadStats();
  }
});
</script>
