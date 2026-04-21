<template>
  <div class="bg-white dark:bg-[var(--bg-card)] rounded-lg border border-gray-200 dark:border-[var(--border-default)] shadow-sm p-6 lg:col-span-1">
    <div class="flex items-center justify-between mb-6">
      <div class="flex items-center space-x-2">
        <ApiOutlined class="text-lg text-slate-700 dark:text-[var(--text-secondary)]" />
        <h3 class="text-lg font-medium text-slate-800 dark:text-[var(--text-primary)]">监听器与协议</h3>
      </div>
      <a-button type="link" size="small" class="px-0" @click="router.push('/listener')">管理监听器 <RightOutlined class="text-[10px]" /></a-button>
    </div>
    
    <div class="flex mb-5">
      <div class="flex-1 text-center border-r border-slate-200 dark:border-slate-700">
        <div class="text-xs text-slate-500 mb-1">已启用</div>
        <div class="text-xl font-semibold text-slate-800 dark:text-[var(--text-primary)]">{{ stats.listeners.enabled }}</div>
      </div>
      <div class="flex-1 text-center border-r border-slate-200 dark:border-slate-700">
        <div class="text-xs text-slate-500 mb-1">已停用</div>
        <div class="text-xl font-semibold text-slate-800 dark:text-[var(--text-primary)]">{{ stats.listeners.disabled }}</div>
      </div>
      <div class="flex-1 text-center">
        <div class="text-xs text-red-500 mb-1">异常</div>
        <div class="text-xl font-semibold text-red-600">{{ stats.listeners.error }}</div>
      </div>
    </div>

    <div class="text-sm font-medium text-slate-700 dark:text-[var(--text-secondary)] mb-3">协议分布</div>
    <div class="space-y-3">
      <div class="flex items-center justify-between">
        <div class="flex items-center space-x-2 text-sm text-slate-600 dark:text-[var(--text-secondary)]">
          <div class="w-1.5 h-4 bg-purple-400 rounded-sm"></div>
          <span>TCP JSON</span>
        </div>
        <span class="text-slate-700 dark:text-[var(--text-secondary)] bg-slate-100 dark:bg-slate-800 px-2 py-0.5 rounded text-xs">{{ stats.listeners.by_kind.tcp_json }}</span>
      </div>
      <div class="flex items-center justify-between">
        <div class="flex items-center space-x-2 text-sm text-slate-600 dark:text-[var(--text-secondary)]">
          <div class="w-1.5 h-4 bg-blue-400 rounded-sm"></div>
          <span>HTTPS JSON</span>
        </div>
        <span class="text-slate-700 dark:text-[var(--text-secondary)] bg-slate-100 dark:bg-slate-800 px-2 py-0.5 rounded text-xs">{{ stats.listeners.by_kind.https_json }}</span>
      </div>
      <div class="flex items-center justify-between">
        <div class="flex items-center space-x-2 text-sm text-slate-600 dark:text-[var(--text-secondary)]">
          <div class="w-1.5 h-4 bg-orange-400 rounded-sm"></div>
          <span>Private Proto</span>
        </div>
        <span class="text-slate-700 dark:text-[var(--text-secondary)] bg-slate-100 dark:bg-slate-800 px-2 py-0.5 rounded text-xs">{{ stats.listeners.by_kind.private_proto }}</span>
      </div>
    </div>
    
    <div class="mt-5 text-xs text-right text-slate-400">
      数据快照于: {{ new Date(stats.generated_at).toLocaleTimeString() }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router';
import { ApiOutlined, RightOutlined } from '@ant-design/icons-vue';
import type { DashboardStats } from '@/api/dashboard';

defineProps<{ stats: DashboardStats }>();
const router = useRouter();
</script>
