<template>
  <a-config-provider :theme="themeConfig">
    <router-view />
  </a-config-provider>
</template>

<script setup lang="ts">
import { theme } from 'ant-design-vue';
import { useEventStore } from '@/store/events';

// Initialize event store to start watching connection profile changes
const eventStore = useEventStore(); 

const themeConfig = {
  algorithm: theme.defaultAlgorithm,
  token: {
    colorPrimary: '#0960bd',
  },
};
</script>
