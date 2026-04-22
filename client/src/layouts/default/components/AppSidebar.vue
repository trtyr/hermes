<template>
  <a-layout-sider
    v-model:collapsed="appStore.sidebarCollapsed"
    :trigger="null"
    collapsible
    theme="dark"
    width="200"
    collapsedWidth="64"
    class="h-screen z-20 relative border-r border-[var(--border-default)]"
    style="background: var(--bg-sidebar)"
  >
    <AppLogo />
    <a-menu
      v-model:selectedKeys="selectedKeys"
      theme="dark"
      mode="inline"
      @click="handleMenuClick"
      :style="{ background: 'transparent' }"
      class="h-[calc(100vh-48px)] overflow-y-auto overflow-x-hidden pt-2 !border-none"
    >
      <a-menu-item key="dashboard">
        <template #icon><DashboardOutlined /></template>
        <span>总览</span>
      </a-menu-item>
      <a-menu-item key="agent">
        <template #icon><RobotOutlined /></template>
        <span>Agent管理</span>
      </a-menu-item>
      <a-menu-item key="listener">
        <template #icon><ApiOutlined /></template>
        <span>监听器管理</span>
      </a-menu-item>
      <a-menu-item key="payload">
        <template #icon><CodeOutlined /></template>
        <span>载荷生成</span>
      </a-menu-item>
      <a-menu-item key="log">
        <template #icon><FileTextOutlined /></template>
        <span>操作日志</span>
      </a-menu-item>
    </a-menu>
  </a-layout-sider>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { useAppStore } from '@/store/app';
import AppLogo from './AppLogo.vue';
import {
  ApiOutlined,
  CodeOutlined,
  DashboardOutlined,
  FileTextOutlined,
  RobotOutlined,
} from '@ant-design/icons-vue';

const router = useRouter();
const route = useRoute();
const appStore = useAppStore();
const selectedKeys = ref<string[]>(['dashboard']);

watch(
  () => route.path,
  (newPath) => {
    const key = newPath.split('/')[1] || 'dashboard';
    selectedKeys.value = [key];
  },
  { immediate: true }
);

const handleMenuClick = ({ key }: { key: string }) => {
  router.push(`/${key}`);
};
</script>
