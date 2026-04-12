<template>
  <div
    class="h-[48px] flex items-center px-4 cursor-pointer select-none overflow-hidden transition-all duration-300"
    @click="router.push('/dashboard')"
  >
    <svg
      class="w-7 h-7 text-blue-500 flex-shrink-0 transition-all duration-300"
      :class="appStore.sidebarCollapsed ? 'mx-auto' : ''"
      fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"
    >
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path>
    </svg>
    <span
      v-show="!appStore.sidebarCollapsed"
      class="text-[15px] font-bold tracking-wide text-white ml-3 whitespace-nowrap transition-opacity duration-300"
    >
      Hermes C2
    </span>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router';
import { useAppStore } from '@/store/app';

const router = useRouter();
const appStore = useAppStore();
</script>
