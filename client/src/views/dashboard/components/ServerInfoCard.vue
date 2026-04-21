<template>
  <div class="bg-white dark:bg-[var(--bg-card)] rounded-lg border border-gray-200 dark:border-[var(--border-default)] shadow-sm p-6 lg:col-span-1">
    <div class="flex items-center space-x-2 mb-6">
      <DesktopOutlined class="text-lg text-slate-700 dark:text-[var(--text-secondary)]" />
      <h3 class="text-lg font-medium text-slate-800 dark:text-[var(--text-primary)]">服务器状态</h3>
    </div>
    
    <div class="space-y-5">
      <div>
        <div class="text-sm text-slate-500 dark:text-[var(--text-secondary)] mb-1">主机信息</div>
        <div class="text-base text-slate-800 dark:text-[var(--text-primary)] font-medium break-all">
          {{ stats.server.hostname || '未知主机名' }}
        </div>
        <div class="text-sm text-slate-500 dark:text-[var(--text-secondary)] mt-0.5">
          {{ stats.server.os_name || 'OS' }} {{ stats.server.os_version || '' }}
          <span v-if="stats.server.kernel_version" class="text-xs ml-1 opacity-70">({{ stats.server.kernel_version }})</span>
        </div>
      </div>
      
      <a-divider style="margin: 12px 0" class="dark:border-[var(--border-default)]" />
      
      <div>
        <div class="flex justify-between items-end mb-2">
          <div class="text-sm text-slate-500 dark:text-[var(--text-secondary)]">内存使用 ({{ formatBytes(stats.server.memory.used_bytes) }} / {{ formatBytes(stats.server.memory.total_bytes) }})</div>
          <div class="text-sm font-medium" :class="memoryPercent > 80 ? 'text-red-500' : memoryPercent > 60 ? 'text-orange-500' : 'text-green-500'">
            {{ memoryPercent }}%
          </div>
        </div>
        <a-progress 
          :percent="memoryPercent" 
          :show-info="false" 
          :status="memoryPercent > 80 ? 'exception' : 'normal'"
          :stroke-color="memoryPercent > 80 ? '#ef4444' : memoryPercent > 60 ? '#f97316' : '#22c55e'" 
          class="!m-0"
        />
      </div>

      <div>
        <div class="text-sm text-slate-500 dark:text-[var(--text-secondary)] mb-1">负载均衡 (Load Average)</div>
        <div class="flex justify-between">
          <div class="text-center">
            <div class="text-xs text-slate-400 mb-0.5">1 分钟</div>
            <div class="text-base font-medium text-slate-700 dark:text-[var(--text-secondary)]">{{ stats.server.load_average.one.toFixed(2) }}</div>
          </div>
          <div class="text-center">
            <div class="text-xs text-slate-400 mb-0.5">5 分钟</div>
            <div class="text-base font-medium text-slate-700 dark:text-[var(--text-secondary)]">{{ stats.server.load_average.five.toFixed(2) }}</div>
          </div>
          <div class="text-center">
            <div class="text-xs text-slate-400 mb-0.5">15 分钟</div>
            <div class="text-base font-medium text-slate-700 dark:text-[var(--text-secondary)]">{{ stats.server.load_average.fifteen.toFixed(2) }}</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { DesktopOutlined } from '@ant-design/icons-vue';
import type { DashboardStats } from '@/api/dashboard';
import { formatBytes, calculateMemoryPercent } from '@/utils/format';

const props = defineProps<{ stats: DashboardStats }>();

const memoryPercent = computed(() => {
  return calculateMemoryPercent(props.stats.server.memory.used_bytes, props.stats.server.memory.total_bytes);
});
</script>
