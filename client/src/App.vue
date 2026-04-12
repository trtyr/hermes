<template>
  <a-config-provider :theme="themeConfig">
    <router-view />
  </a-config-provider>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { theme } from 'ant-design-vue';
import { useAppStore } from '@/store/app';
import { useEventStore } from '@/store/events';

const appStore = useAppStore();
// Initialize event store to start watching connection profile changes
const eventStore = useEventStore(); 

const themeConfig = computed(() => ({
  algorithm: appStore.isDark ? theme.darkAlgorithm : theme.defaultAlgorithm,
  token: {
    colorPrimary: '#0960bd',
  },
}));
</script>
