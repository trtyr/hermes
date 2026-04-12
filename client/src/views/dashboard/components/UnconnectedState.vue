<template>
  <div class="flex flex-col items-center justify-center h-full max-w-lg mx-auto text-center">
    <div class="w-16 h-16 bg-blue-50 text-blue-500 rounded-full flex items-center justify-center mb-4">
      <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path>
      </svg>
    </div>
    <h2 class="text-2xl font-semibold text-slate-800 dark:text-slate-100 mb-2">未连接到任何后端服务器</h2>
    <p class="text-slate-500 dark:text-slate-400 mb-8">请配置并选择一个活跃的 Hermes 服务端进行通信。这是客户端与后端交互的必要步骤。</p>
    
    <a-button type="primary" size="large" @click="emit('connect')">
      <template #icon><ApiOutlined /></template>
      立即配置后端连接
    </a-button>
  </div>
</template>

<script setup lang="ts">
import { ApiOutlined } from '@ant-design/icons-vue';

const emit = defineEmits(['connect']);
</script>
