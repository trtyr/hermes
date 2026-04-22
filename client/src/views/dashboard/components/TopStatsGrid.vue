<template>
  <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
    <div class="bg-white p-6 rounded-lg border border-gray-200 shadow-sm flex items-center">
      <div class="p-3 bg-blue-50 text-blue-500 rounded-lg mr-4">
        <RobotOutlined class="text-2xl" />
      </div>
      <div>
        <div class="text-slate-500 text-sm mb-1">活跃节点 (在线/总数)</div>
        <div class="text-2xl font-bold text-slate-800">
          <span class="text-blue-500">{{ stats.agents.online }}</span> / {{ stats.agents.total }}
        </div>
      </div>
    </div>
    
    <div class="bg-white p-6 rounded-lg border border-gray-200 shadow-sm flex items-center">
      <div class="p-3 bg-green-50 text-green-500 rounded-lg mr-4">
        <ApiOutlined class="text-2xl" />
      </div>
      <div>
        <div class="text-slate-500 text-sm mb-1">监听器 (运行中/总数)</div>
        <div class="text-2xl font-bold text-slate-800">
          <span class="text-green-500">{{ stats.listeners.running }}</span> / {{ stats.listeners.total }}
        </div>
      </div>
    </div>
    
    <div class="bg-white p-6 rounded-lg border border-gray-200 shadow-sm flex items-center">
      <div class="p-3 bg-purple-50 text-purple-500 rounded-lg mr-4">
        <HddOutlined class="text-2xl" />
      </div>
      <div>
        <div class="text-slate-500 text-sm mb-1">物理核心 / 架构</div>
        <div class="text-2xl font-bold text-slate-800">
          {{ stats.server.cpu_cores || '?' }} <span class="text-lg font-normal text-slate-500">{{ stats.server.architecture || 'Unknown' }}</span>
        </div>
      </div>
    </div>
    
    <div class="bg-white p-6 rounded-lg border border-gray-200 shadow-sm flex items-center">
      <div class="p-3 bg-orange-50 text-orange-500 rounded-lg mr-4">
        <ClockCircleOutlined class="text-2xl" />
      </div>
      <div>
        <div class="text-slate-500 text-sm mb-1">系统已运行时间</div>
        <div class="text-xl font-bold text-slate-800 truncate">
          {{ formatUptime(stats.server.uptime_seconds) }}
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { RobotOutlined, ApiOutlined, HddOutlined, ClockCircleOutlined } from '@ant-design/icons-vue';
import type { DashboardStats } from '@/api/dashboard';
import { formatUptime } from '@/utils/format';

defineProps<{ stats: DashboardStats }>();
</script>
