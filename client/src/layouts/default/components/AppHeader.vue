<template>
  <header class="top-0 flex w-full flex-[0_0_auto] items-center border-b border-gray-200 bg-white pl-2 transition-[margin-top] duration-200" style="height: 50px;">
    <!-- Toggle Sidebar Button -->
    <button type="button" class="inline-flex items-center justify-center whitespace-nowrap font-medium transition-colors h-8 w-8 text-lg text-slate-500 hover:bg-slate-100 rounded-md mr-1" @click="appStore.toggleCollapse">
      <svg v-if="appStore.sidebarCollapsed" aria-hidden="true" viewBox="0 0 1024 1024" class="size-4 fill-current"><path d="M128 192h768v128H128zm256 256h512v128H384zm-256 256h768v128H128zm576-320 192 128-192 128z"></path></svg>
      <svg v-else aria-hidden="true" viewBox="0 0 1024 1024" class="size-4 fill-current"><path d="M896 192H128v128h768zm0 256H384v128h512zm0 256H128v128h768zM320 384L128 512l192 128z"></path></svg>
    </button>

    <!-- Content Refresh Button -->
    <button type="button" class="inline-flex items-center justify-center whitespace-nowrap font-medium transition-colors h-8 w-8 text-lg text-slate-500 hover:bg-slate-100 rounded-md mr-1" @click="emit('refresh')">
      <svg aria-hidden="true" viewBox="0 0 24 24" class="size-4 fill-none stroke-current stroke-2" :class="{ 'animate-spin': isRefreshing }">
        <path stroke-linecap="round" stroke-linejoin="round" d="M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8"></path>
        <path stroke-linecap="round" stroke-linejoin="round" d="M21 3v5h-5"></path>
      </svg>
    </button>

    <!-- Breadcrumb -->
    <div class="hidden lg:flex items-center mt-[2px]">
      <nav class="ml-2 flex flex-wrap items-center gap-1.5 text-sm m-0 leading-none">
        <router-link class="text-slate-500 hover:text-slate-900 transition-colors flex items-center leading-none no-underline" to="/dashboard">
          <svg viewBox="0 0 24 24" class="mr-1 size-4 fill-none stroke-current stroke-2"><rect x="3" y="3" width="7" height="9" rx="1"></rect><rect x="14" y="3" width="7" height="5" rx="1"></rect><rect x="14" y="12" width="7" height="9" rx="1"></rect><rect x="3" y="16" width="7" height="5" rx="1"></rect></svg>
          <span class="translate-y-[0.5px]">概览</span>
        </router-link>
        <span class="text-slate-300 flex items-center">
          <svg viewBox="0 0 24 24" class="size-3.5 fill-none stroke-current stroke-2"><path stroke-linecap="round" stroke-linejoin="round" d="m9 18 6-6-6-6"></path></svg>
        </span>
        <span class="text-slate-900 flex items-center leading-none">
          <span class="mr-1 text-[16px] leading-none flex items-center"><component :is="currentRouteIcon" /></span>
          <span class="translate-y-[0.5px]">{{ routeMetaTitle }}</span>
        </span>
      </nav>
    </div>

    <div class="flex-1"></div>

    <!-- Right Side Tools -->
    <div class="flex items-center">
      <NotificationCenter>
        <button class="menu-btn relative mx-1">
          <span v-if="notificationStore.unreadCount > 0" class="absolute top-1.5 right-1.5 size-2 rounded-sm bg-[#0960bd]"></span>
          <svg viewBox="0 0 24 24" class="size-4 fill-none stroke-current stroke-2"><path stroke-linecap="round" stroke-linejoin="round" d="M10.268 21a2 2 0 0 0 3.464 0M3.262 15.326A1 1 0 0 0 4 17h16a1 1 0 0 0 .74-1.673C19.41 13.956 18 12.499 18 8A6 6 0 0 0 6 8c0 4.499-1.411 5.956-2.738 7.326"></path></svg>
        </button>
      </NotificationCenter>

      <!-- Profile Dropdown -->
      <a-dropdown placement="bottomRight">
        <div class="mr-2 cursor-pointer rounded-full p-1.5 hover:bg-slate-100">
          <div class="relative flex items-center h-8 w-8 shrink-0">
            <a-avatar size="small" class="!h-8 !w-8 bg-slate-200 text-slate-700 rounded-full">
              <template #icon><UserOutlined class="text-[13px]" /></template>
            </a-avatar>
            <span class="absolute right-0 bottom-0 h-3 w-3 rounded-full border-2 border-white bg-green-500"></span>
          </div>
        </div>
        <template #overlay>
          <a-menu>
            <a-menu-item key="logout" @click="handleLogout"><template #icon><LogoutOutlined /></template>退出登录</a-menu-item>
          </a-menu>
        </template>
      </a-dropdown>
    </div>
  </header>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { useAppStore } from '@/store/app';
import { useConnectionStore } from '@/store/connection';
import { useNotificationStore } from '@/store/notifications';
import NotificationCenter from './NotificationCenter.vue';
import {
  DashboardOutlined, RobotOutlined, ApiOutlined, CodeOutlined, FileTextOutlined,
  UserOutlined, LogoutOutlined
} from '@ant-design/icons-vue';

const props = defineProps<{ isRefreshing: boolean }>();
const emit = defineEmits(['refresh']);

const router = useRouter();
const route = useRoute();
const appStore = useAppStore();
const connectionStore = useConnectionStore();
const notificationStore = useNotificationStore();

const routeIcons = {
  dashboard: DashboardOutlined,
  agent: RobotOutlined,
  listener: ApiOutlined,
  payload: CodeOutlined,
  log: FileTextOutlined,
} as const;

const routeMetaTitle = computed(() => route.meta?.title?.toString().split(' - ')[0] || '页面');
const currentRouteKey = computed(() => (route.path.split('/')[1] || 'dashboard') as keyof typeof routeIcons);
const currentRouteIcon = computed(() => routeIcons[currentRouteKey.value] || DashboardOutlined);

const handleLogout = () => {
  connectionStore.logout();
  router.push('/login');
};
</script>

<style scoped>
@reference "tailwindcss";
.menu-btn {
  @apply inline-flex items-center justify-center h-8 w-8 text-lg rounded-full text-slate-500 hover:bg-slate-100 hover:text-slate-900 mr-1 transition-colors;
}
</style>
