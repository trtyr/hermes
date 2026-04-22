<template>
  <a-layout class="h-screen overflow-hidden transition-colors duration-300">
    <!-- Main Left Sidebar -->
    <AppSidebar />

    <a-layout class="h-screen bg-[#f0f2f5] flex flex-col">
      <!-- Top Action Header -->
      <AppHeader :isRefreshing="refreshing" @refresh="refreshCurrentView" />

      <!-- Breadcrumb Tabs Navigation -->
      <AppTabs />

      <!-- Dynamic Content Page -->
      <a-layout-content class="flex-1 w-full p-4 overflow-hidden relative flex flex-col">
        <div class="flex-1 w-full bg-white rounded-md shadow-sm border border-gray-200 overflow-y-auto relative">
          <router-view v-slot="{ Component }">
            <transition name="fade-slide" mode="out-in">
              <keep-alive>
                <component :is="Component" :key="route.path + contentRefreshKey" />
              </keep-alive>
            </transition>
          </router-view>
        </div>
      </a-layout-content>
    </a-layout>
  </a-layout>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useRoute } from 'vue-router';
import AppSidebar from './components/AppSidebar.vue';
import AppHeader from './components/AppHeader.vue';
import AppTabs from './components/AppTabs.vue';

const route = useRoute();
const contentRefreshKey = ref(0);
const refreshing = ref(false);

const refreshCurrentView = () => {
  refreshing.value = true;
  contentRefreshKey.value += 1;
  window.setTimeout(() => {
    refreshing.value = false;
  }, 450);
};
</script>

<style scoped>
.fade-slide-enter-active,
.fade-slide-leave-active {
  transition: opacity 0.25s ease, transform 0.25s ease;
}

.fade-slide-enter-from {
  opacity: 0;
  transform: translateX(-10px);
}

.fade-slide-leave-to {
  opacity: 0;
  transform: translateX(10px);
}
</style>
