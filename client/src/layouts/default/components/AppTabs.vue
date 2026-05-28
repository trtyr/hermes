<template>
  <div class="flex-shrink-0 w-full bg-slate-100 border-b border-gray-200 pt-2 px-2 flex items-end space-x-1 overflow-x-auto z-10 transition-colors duration-300 h-[42px]" role="tablist">
    <button
      v-for="tab in appStore.visitedViews"
      :key="tab.path"
      type="button"
      role="tab"
      :aria-selected="route.path === tab.path"
      @click="router.push(tab.path)"
      :class="[
        'group relative flex items-center h-[34px] px-4 min-w-[120px] max-w-[200px] cursor-pointer select-none transition-all duration-200',
        'rounded-t-lg mx-[-1px] border-0 bg-transparent', 
        route.path === tab.path
          ? 'bg-white text-primary font-medium z-10'
          : 'bg-transparent text-slate-500 hover:text-slate-700 hover:bg-slate-200/50'
      ]"
    >
      <!-- Separator (inactive) -->
      <div v-if="route.path !== tab.path" class="absolute right-0 top-1/2 -translate-y-1/2 w-[1px] h-4 bg-slate-300 group-hover:hidden"></div>
      
      <!-- Active Top Highlight -->
      <div v-if="route.path === tab.path" class="absolute top-0 left-0 w-full h-[2px] bg-primary rounded-t-lg"></div>

      <!-- Icon & Text -->
      <component 
        :is="routeIcons[(tab.path.split('/')[1] || 'dashboard') as keyof typeof routeIcons] || DashboardOutlined" 
        class="mr-2 text-[14px]"
        :class="route.path === tab.path ? 'text-primary' : 'text-slate-400'"
      />
      <span class="flex-1 truncate text-xs">{{ tab.title }}</span>
      
      <!-- Close Button -->
      <CloseOutlined
        v-if="appStore.visitedViews.length > 1"
        @click.stop="closeTab(tab.path)"
        class="ml-2 text-[10px] p-0.5 rounded-full hover:bg-slate-200 text-slate-400 hover:text-red-500 transition-all opacity-0 group-hover:opacity-100"
        :class="route.path === tab.path ? '!opacity-100' : ''"
      />
    </button>
  </div>
</template>

<script setup lang="ts">
import { watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { useAppStore } from '@/store/app';
import {
  DashboardOutlined, RobotOutlined, ApiOutlined, CodeOutlined, FileTextOutlined, CloseOutlined
} from '@ant-design/icons-vue';

const routeIcons = {
  dashboard: DashboardOutlined,
  agent: RobotOutlined,
  listener: ApiOutlined,
  payload: CodeOutlined,
  log: FileTextOutlined,
};

const route = useRoute();
const router = useRouter();
const appStore = useAppStore();

watch(
  () => route.path,
  () => {
    if (route.name && route.meta?.title) {
      appStore.addView({
        path: route.path,
        name: route.name as string,
        title: (route.meta.title as string).split(' - ')[0]
      });
    }
  },
  { immediate: true }
);

const closeTab = (path: string) => {
  appStore.removeView(path);
  if (route.path === path) {
    const lastView = appStore.visitedViews[appStore.visitedViews.length - 1];
    if (lastView) router.push(lastView.path);
    else router.push('/dashboard');
  }
};
</script>
