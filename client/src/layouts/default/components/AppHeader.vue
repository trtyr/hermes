<template>
  <header class="top-0 flex w-full flex-[0_0_auto] items-center border-b border-gray-200 bg-white pl-2 transition-[margin-top] duration-200 dark:border-[#14161A] dark:bg-[#1C1E22]" style="height: 50px;">
    <!-- Toggle Sidebar Button -->
    <button type="button" class="inline-flex items-center justify-center whitespace-nowrap font-medium transition-colors h-8 w-8 text-lg text-slate-500 hover:bg-slate-100 dark:text-slate-400 dark:hover:bg-[#2A2D33] rounded-md mr-1" @click="appStore.toggleCollapse">
      <svg v-if="appStore.sidebarCollapsed" aria-hidden="true" viewBox="0 0 1024 1024" class="size-4 fill-current"><path d="M128 192h768v128H128zm256 256h512v128H384zm-256 256h768v128H128zm576-320 192 128-192 128z"></path></svg>
      <svg v-else aria-hidden="true" viewBox="0 0 1024 1024" class="size-4 fill-current"><path d="M896 192H128v128h768zm0 256H384v128h512zm0 256H128v128h768zM320 384L128 512l192 128z"></path></svg>
    </button>

    <!-- Content Refresh Button -->
    <button type="button" class="inline-flex items-center justify-center whitespace-nowrap font-medium transition-colors h-8 w-8 text-lg text-slate-500 hover:bg-slate-100 dark:text-slate-400 dark:hover:bg-[#2A2D33] rounded-md mr-1" @click="emit('refresh')">
      <svg aria-hidden="true" viewBox="0 0 24 24" class="size-4 fill-none stroke-current stroke-2" :class="{ 'animate-spin': isRefreshing }">
        <path stroke-linecap="round" stroke-linejoin="round" d="M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8"></path>
        <path stroke-linecap="round" stroke-linejoin="round" d="M21 3v5h-5"></path>
      </svg>
    </button>

    <!-- Breadcrumb -->
    <div class="hidden lg:flex items-center mt-[2px]">
      <nav class="ml-2 flex flex-wrap items-center gap-1.5 text-sm m-0 leading-none">
        <a class="text-slate-500 hover:text-slate-900 dark:hover:text-slate-100 transition-colors flex items-center leading-none" href="javascript:void 0" @click.prevent="router.push('/dashboard')">
          <svg viewBox="0 0 24 24" class="mr-1 size-4 fill-none stroke-current stroke-2"><rect x="3" y="3" width="7" height="9" rx="1"></rect><rect x="14" y="3" width="7" height="5" rx="1"></rect><rect x="14" y="12" width="7" height="9" rx="1"></rect><rect x="3" y="16" width="7" height="5" rx="1"></rect></svg>
          <span class="translate-y-[0.5px]">概览</span>
        </a>
        <span class="text-slate-300 dark:text-slate-600 flex items-center">
          <svg viewBox="0 0 24 24" class="size-3.5 fill-none stroke-current stroke-2"><path stroke-linecap="round" stroke-linejoin="round" d="m9 18 6-6-6-6"></path></svg>
        </span>
        <span class="text-slate-900 dark:text-slate-100 flex items-center leading-none">
          <span class="mr-1 text-[16px] leading-none flex items-center"><component :is="currentRouteIcon" /></span>
          <span class="translate-y-[0.5px]">{{ routeMetaTitle }}</span>
        </span>
      </nav>
    </div>

    <div class="flex-1"></div>

    <!-- Right Side Tools -->
    <div class="flex items-center">
      <div class="mr-1 sm:mr-4 flex items-center gap-3 cursor-pointer rounded-2xl md:bg-slate-100 dark:md:bg-[#2A2D33] px-2 py-0.5 text-slate-500 hover:text-slate-900 dark:text-slate-400 dark:hover:text-slate-100" @click="openAction('全局搜索')">
        <svg viewBox="0 0 24 24" class="size-4 fill-none stroke-current stroke-2"><path stroke-linecap="round" stroke-linejoin="round" d="m21 21-4.34-4.34"></path><circle cx="11" cy="11" r="8"></circle></svg>
        <span class="hidden md:block text-xs">搜索</span>
        <span class="hidden md:block border bg-white dark:bg-[#1C1E22] border-slate-300 dark:border-slate-600 px-1.5 py-1 text-xs rounded-r-xl">⌘ K</span>
      </div>

      <button class="menu-btn" @click="openAction('偏好设置')">
        <svg viewBox="0 0 24 24" class="size-4 fill-none stroke-current stroke-2"><path stroke-linecap="round" stroke-linejoin="round" d="M9.671 4.136a2.34 2.34 0 0 1 4.659 0 2.34 2.34 0 0 0 3.319 1.915 2.34 2.34 0 0 1 2.33 4.033 2.34 2.34 0 0 0 0 3.831 2.34 2.34 0 0 1-2.33 4.033 2.34 2.34 0 0 0-3.319 1.915 2.34 2.34 0 0 1-4.659 0 2.34 2.34 0 0 0-3.32-1.915 2.34 2.34 0 0 1-2.33-4.033 2.34 2.34 0 0 0 0-3.831A2.34 2.34 0 0 1 6.35 6.051a2.34 2.34 0 0 0 3.319-1.915"></path><circle cx="12" cy="12" r="3"></circle></svg>
      </button>

      <button class="menu-btn" @click="appStore.toggleTheme">
        <svg v-if="!appStore.isDark" viewBox="0 0 24 24" class="size-4"><circle cx="12" cy="12" r="5" class="fill-current"></circle><g class="stroke-current opacity-90" stroke-width="2"><line x1="12" y1="1" x2="12" y2="3"></line><line x1="12" y1="21" x2="12" y2="23"></line><line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line><line x1="1" y1="12" x2="3" y2="12"></line><line x1="21" y1="12" x2="23" y2="12"></line><line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line></g></svg>
        <svg v-else viewBox="0 0 24 24" class="size-4 fill-none stroke-current stroke-2"><path stroke-linecap="round" stroke-linejoin="round" d="M20.354 15.354A9 9 0 0 1 8.646 3.646 9.003 9.003 0 0 0 12 21a9.003 9.003 0 0 0 8.354-5.646z"></path></svg>
      </button>

      <button class="menu-btn" @click="openAction('多语言切换')"><svg viewBox="0 0 24 24" class="size-4 fill-none stroke-current stroke-2"><path stroke-linecap="round" stroke-linejoin="round" d="m5 8 6 6m-7 0 6-6 2-3M2 5h12M7 2h1m14 20-5-10-5 10M14 18h6"></path></svg></button>
      
      <button class="menu-btn" @click="openAction('小部件')"><svg viewBox="0 0 2048 2048" class="size-4 fill-current"><path d="M896 768H512V256h128v384h256zm1152 640q0 87-22 168t-64 152t-100 130t-128 101t-152 66t-168 23q-134 0-251-49t-205-136t-139-204t-51-251q0-132 50-248t138-204t203-137t249-51q132 0 248 50t204 138t137 203t51 249m-640 512q21 0 37-15t29-40t21-53t15-58t9-53t5-37h-230q1 13 5 37t10 52t15 58t21 54t27 39t36 16m125-384q3-64 3-128q0-63-3-128h-250q-3 65-3 128q0 64 3 128zm-637-128q0 32 4 64t12 64h243q-6-128 0-256H912q-8 32-12 64t-4 64m512-512q-19 0-34 15t-27 40t-21 54t-15 58t-11 53t-5 36h225q-1-11-5-34t-11-52t-16-59t-21-54t-27-41t-32-16m253 384q3 64 3 128t-2 128h242q8-32 12-64t4-64t-4-64t-12-64zm190-128q-43-75-108-131t-145-89q20 53 32 108t20 112zm-637-218q-78 32-142 88t-107 130h200q7-56 18-110t31-108m-249 730q42 73 105 129t142 88q-20-52-30-107t-17-110zm643 215q77-32 139-87t104-128h-198q-5 55-15 109t-30 106M640 0q88 0 170 23t153 64t129 100t100 130t65 153t23 170h-128q0-106-40-199t-110-162t-163-110t-199-41t-199 40t-162 110t-110 163t-41 199t40 199t110 162t163 110t199 41v128q-88 0-170-23t-153-64t-129-100T88 963T23 810T0 640q0-132 50-248t138-204T391 51T640 0"></path></svg></button>

      <button class="menu-btn" @click="toggleFullscreen"><svg viewBox="0 0 24 24" class="size-4 fill-none stroke-current stroke-2"><path stroke-linecap="round" stroke-linejoin="round" d="M8 3H5a2 2 0 0 0-2 2v3M21 8V5a2 2 0 0 0-2-2h-3M3 16v3a2 2 0 0 0 2 2h3M16 21h3a2 2 0 0 0 2-2v-3"></path></svg></button>

      <button class="menu-btn relative mx-1" @click="openAction('通知中心')">
        <span class="absolute top-1.5 right-1.5 size-2 rounded-sm bg-[#0960bd]"></span>
        <svg viewBox="0 0 24 24" class="size-4 fill-none stroke-current stroke-2"><path stroke-linecap="round" stroke-linejoin="round" d="M10.268 21a2 2 0 0 0 3.464 0M3.262 15.326A1 1 0 0 0 4 17h16a1 1 0 0 0 .74-1.673C19.41 13.956 18 12.499 18 8A6 6 0 0 0 6 8c0 4.499-1.411 5.956-2.738 7.326"></path></svg>
      </button>

      <!-- Profile Dropdown -->
      <a-dropdown placement="bottomRight">
        <div class="mr-2 cursor-pointer rounded-full p-1.5 hover:bg-slate-100 dark:hover:bg-[#2A2D33]">
          <div class="relative flex items-center h-8 w-8 shrink-0">
            <a-avatar size="small" class="!h-8 !w-8 bg-slate-200 text-slate-700 dark:bg-slate-700 dark:text-slate-100 rounded-full">
              <template #icon><UserOutlined class="text-[13px]" /></template>
            </a-avatar>
            <span class="absolute right-0 bottom-0 h-3 w-3 rounded-full border-2 border-white bg-green-500 dark:border-[#1C1E22]"></span>
          </div>
        </div>
        <template #overlay>
          <a-menu>
            <a-menu-item key="preferences" @click="openAction('偏好设置')"><template #icon><SettingOutlined /></template>控制台设置</a-menu-item>
            <a-menu-item key="logout" @click="router.push('/login')"><template #icon><LogoutOutlined /></template>退出 UI 会话</a-menu-item>
          </a-menu>
        </template>
      </a-dropdown>
    </div>
  </header>
</template>

<script setup lang="ts">
import { computed, onMounted, onBeforeUnmount } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { message } from 'ant-design-vue';
import { useAppStore } from '@/store/app';
import {
  DashboardOutlined, RobotOutlined, ApiOutlined, CodeOutlined, FileTextOutlined,
  UserOutlined, SettingOutlined, LogoutOutlined
} from '@ant-design/icons-vue';

const props = defineProps<{ isRefreshing: boolean }>();
const emit = defineEmits(['refresh']);

const router = useRouter();
const route = useRoute();
const appStore = useAppStore();

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

const toggleFullscreen = async () => {
  try {
    if (!document.fullscreenElement) await document.documentElement.requestFullscreen();
    else await document.exitFullscreen();
  } catch (err) {
    message.error('不支持全屏');
  }
};

const openAction = (name: string) => {
  message.info(`${name} 入口已预留`);
};
</script>

<style scoped>
@reference "tailwindcss";
.menu-btn {
  @apply inline-flex items-center justify-center h-8 w-8 text-lg rounded-full text-slate-500 hover:bg-slate-100 hover:text-slate-900 dark:text-slate-400 dark:hover:bg-[#2A2D33] dark:hover:text-slate-100 mr-1 transition-colors;
}
</style>
