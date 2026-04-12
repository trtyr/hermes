<template>
  <div class="bg-white dark:bg-[#1C1E22] rounded-lg border border-gray-200 dark:border-[#14161A] shadow-sm p-6 lg:col-span-1">
    <div class="flex items-center justify-between mb-6">
      <div class="flex items-center space-x-2">
        <RobotOutlined class="text-lg text-slate-700 dark:text-slate-300" />
        <h3 class="text-lg font-medium text-slate-800 dark:text-slate-100">受控节点分布</h3>
      </div>
      <a-button type="link" size="small" class="px-0" @click="router.push('/agent')">管理节点 <RightOutlined class="text-[10px]" /></a-button>
    </div>
    
    <div class="space-y-4">
      <div class="flex items-center justify-between p-3 bg-slate-50 dark:bg-[#14161A] rounded-lg">
        <div class="flex items-center space-x-3">
          <div class="w-3 h-3 rounded-full bg-green-500"></div>
          <span class="text-slate-700 dark:text-slate-300">在线 (Online)</span>
        </div>
        <span class="font-semibold text-lg text-slate-800 dark:text-slate-100">{{ stats.agents.online }}</span>
      </div>
      
      <div class="flex items-center justify-between p-3 bg-slate-50 dark:bg-[#14161A] rounded-lg">
        <div class="flex items-center space-x-3">
          <div class="w-3 h-3 rounded-full bg-slate-400"></div>
          <span class="text-slate-700 dark:text-slate-300">离线 (Offline)</span>
        </div>
        <span class="font-semibold text-lg text-slate-800 dark:text-slate-100">{{ stats.agents.offline }}</span>
      </div>
      
      <div class="flex items-center justify-between p-3 bg-slate-50 dark:bg-[#14161A] rounded-lg">
        <div class="flex items-center space-x-3">
          <div class="w-3 h-3 rounded-full bg-red-500"></div>
          <span class="text-slate-700 dark:text-slate-300">已禁用 (Disabled)</span>
        </div>
        <span class="font-semibold text-lg text-slate-800 dark:text-slate-100">{{ stats.agents.disabled }}</span>
      </div>

      <a-divider style="margin: 12px 0" class="dark:border-[#2A2D33]" />
      
      <div class="flex items-center justify-between p-3 border border-blue-100 dark:border-blue-900/30 bg-blue-50/50 dark:bg-blue-900/10 rounded-lg">
        <div class="flex items-center space-x-3">
          <div class="w-3 h-3 rounded-full border-2 border-blue-500"></div>
          <span class="text-blue-700 dark:text-blue-300 font-medium">当前活跃会话 (Sessions)</span>
        </div>
        <span class="font-bold text-xl text-blue-600 dark:text-blue-400">{{ stats.agents.connected_sessions }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router';
import { RobotOutlined, RightOutlined } from '@ant-design/icons-vue';
import type { DashboardStats } from '@/api/dashboard';

defineProps<{ stats: DashboardStats }>();
const router = useRouter();
</script>
