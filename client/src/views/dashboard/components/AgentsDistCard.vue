<template>
  <div class="bg-white rounded-lg border border-gray-200 shadow-sm p-6 lg:col-span-1">
    <div class="flex items-center justify-between mb-6">
      <div class="flex items-center space-x-2">
        <RobotOutlined class="text-lg text-slate-700" />
        <h3 class="text-lg font-medium text-slate-800">受控节点分布</h3>
      </div>
      <a-button type="link" size="small" class="px-0" @click="router.push('/agent')">管理节点 <RightOutlined class="text-[10px]" /></a-button>
    </div>
    
    <div class="space-y-4">
      <div class="flex items-center justify-between p-3 bg-slate-50 rounded-lg">
        <div class="flex items-center space-x-3">
          <div class="w-3 h-3 rounded-full bg-green-500"></div>
          <span class="text-slate-700">在线 (Online)</span>
        </div>
        <span class="font-semibold text-lg text-slate-800">{{ stats.agents.online }}</span>
      </div>
      
      <div class="flex items-center justify-between p-3 bg-slate-50 rounded-lg">
        <div class="flex items-center space-x-3">
          <div class="w-3 h-3 rounded-full bg-slate-400"></div>
          <span class="text-slate-700">离线 (Offline)</span>
        </div>
        <span class="font-semibold text-lg text-slate-800">{{ stats.agents.offline }}</span>
      </div>
      
      <div class="flex items-center justify-between p-3 bg-slate-50 rounded-lg">
        <div class="flex items-center space-x-3">
          <div class="w-3 h-3 rounded-full bg-red-500"></div>
          <span class="text-slate-700">已禁用 (Disabled)</span>
        </div>
        <span class="font-semibold text-lg text-slate-800">{{ stats.agents.disabled }}</span>
      </div>

      <a-divider style="margin: 12px 0" />
      
      <div class="flex items-center justify-between p-3 border border-blue-100 bg-blue-50/50 rounded-lg">
        <div class="flex items-center space-x-3">
          <div class="w-3 h-3 rounded-full border-2 border-blue-500"></div>
          <span class="text-blue-700 font-medium">当前活跃会话 (Sessions)</span>
        </div>
        <span class="font-bold text-xl text-blue-600">{{ stats.agents.connected_sessions }}</span>
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
