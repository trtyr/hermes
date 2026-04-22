<template>
  <div class="h-full w-full flex flex-col p-4 relative bg-[#f0f2f5] transition-colors duration-300">
    <!-- Header -->
    <div class="flex justify-between items-center mb-4">
      <h2 class="text-xl font-semibold text-slate-800 flex items-center gap-2 m-0">
        <CodeOutlined class="text-blue-500" />
        交互终端 - {{ agentId }}
      </h2>
      <div>
        <a-tag :color="wsConnected ? (sessionId ? 'success' : 'processing') : 'error'" class="border-0 font-medium">
          {{ wsConnected ? (sessionId ? '已连接 (Session Active)' : '初始化会话中...') : 'WebSocket 失联' }}
        </a-tag>
      </div>
    </div>
    
    <!-- Terminal Container -->
    <div class="flex-1 bg-[#1e1e1e] rounded-lg border border-gray-200 shadow-sm overflow-hidden relative">
      <div ref="terminalContainer" class="absolute inset-0 p-3 pt-2"></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRoute } from 'vue-router';
import { CodeOutlined } from '@ant-design/icons-vue';
import { useAppStore } from '@/store/app';
import { useTerminal } from './hooks/useTerminal';
import 'xterm/css/xterm.css';

const route = useRoute();
const appStore = useAppStore();
const agentId = ref(route.params.id as string);

// Sync tab title
onMounted(() => {
  const currentView = appStore.visitedViews.find(v => v.path === route.path);
  if (currentView) {
    currentView.title = `终端: ${agentId.value}`;
  } else {
    appStore.addView({
      path: route.path,
      name: route.name as string || 'AgentTerminal',
      title: `终端: ${agentId.value}`
    });
  }
});

// Delegate complex behavior to the Microkernel Composable Hook
const { terminalContainer, sessionId, wsConnected } = useTerminal(agentId.value);
</script>

<style>
.xterm .xterm-viewport {
  overflow-y: auto !important;
}
.xterm .xterm-viewport::-webkit-scrollbar {
  width: 8px;
}
.xterm .xterm-viewport::-webkit-scrollbar-track {
  background: #111;
}
.xterm .xterm-viewport::-webkit-scrollbar-thumb {
  background: #555;
  border-radius: 4px;
}
</style>
