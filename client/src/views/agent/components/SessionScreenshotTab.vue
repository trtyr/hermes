<template>
  <div class="flex flex-col items-center justify-center h-full min-h-[300px]">
    <template v-if="screenshotLoading">
      <a-spin size="large" />
      <p class="mt-4 text-sm text-slate-400">正在截图...</p>
    </template>
    <template v-else-if="screenshotUrl">
      <img :src="screenshotUrl" alt="Agent screenshot" class="max-w-full max-h-[60vh] rounded border border-gray-200 shadow-sm" />
      <a-button class="mt-4" @click="doScreenshot">
        <template #icon><CameraOutlined /></template>
        重新截图
      </a-button>
    </template>
    <template v-else>
      <CameraOutlined style="font-size: 48px; opacity: 0.3; color: #8c8c8c" />
      <p class="mt-4 text-sm text-slate-400">点击下方按钮截取目标屏幕</p>
      <a-button type="primary" class="mt-2" :disabled="!isOnline" @click="doScreenshot">
        <template #icon><CameraOutlined /></template>
        截取屏幕
      </a-button>
    </template>
  </div>
</template>

<script setup lang="ts">
import { ref, onBeforeUnmount } from 'vue';
import { message } from 'ant-design-vue';
import { CameraOutlined } from '@ant-design/icons-vue';
import { takeScreenshot } from '@/api/agent';
import { useEventStore } from '@/store/events';

const props = defineProps<{ agentId: string; isOnline: boolean }>();

const screenshotLoading = ref(false);
const screenshotUrl = ref<string | null>(null);
const pendingScreenshotTaskId = ref<string | null>(null);
let unsubscribeEvents: (() => void) | null = null;
let screenshotTimeoutId: ReturnType<typeof setTimeout> | null = null;

const eventStore = useEventStore();

async function doScreenshot() {
  screenshotLoading.value = true;
  screenshotUrl.value = null;
  pendingScreenshotTaskId.value = null;

  if (screenshotTimeoutId) {
    clearTimeout(screenshotTimeoutId);
    screenshotTimeoutId = null;
  }
  if (unsubscribeEvents) {
    unsubscribeEvents();
    unsubscribeEvents = null;
  }

  try {
    const res = await takeScreenshot(props.agentId);
    if (res.success && res.task_id) {
      pendingScreenshotTaskId.value = res.task_id;

      unsubscribeEvents = eventStore.subscribe((event) => {
        if (event.type !== 'task_result') return;
        if (event.task_id !== pendingScreenshotTaskId.value) return;

        if (screenshotTimeoutId) {
          clearTimeout(screenshotTimeoutId);
          screenshotTimeoutId = null;
        }

        screenshotLoading.value = false;
        pendingScreenshotTaskId.value = null;

        const { success, output } = event;

        if (success && output) {
          screenshotUrl.value = `data:image/png;base64,${output}`;
        } else {
          message.error(output || '截图失败');
        }

        if (unsubscribeEvents) {
          unsubscribeEvents();
          unsubscribeEvents = null;
        }
      });

      screenshotTimeoutId = setTimeout(() => {
        screenshotTimeoutId = null;
        if (unsubscribeEvents) {
          unsubscribeEvents();
          unsubscribeEvents = null;
        }
        pendingScreenshotTaskId.value = null;
        screenshotLoading.value = false;
        message.error('截图超时，请重试');
      }, 60_000);
    } else {
      screenshotLoading.value = false;
      message.error(res.detail || '截图任务下发失败');
    }
  } catch (e: any) {
    screenshotLoading.value = false;
    message.error(e.message);
  }
}

onBeforeUnmount(() => {
  if (screenshotTimeoutId) {
    clearTimeout(screenshotTimeoutId);
    screenshotTimeoutId = null;
  }
  if (unsubscribeEvents) {
    unsubscribeEvents();
    unsubscribeEvents = null;
  }
  pendingScreenshotTaskId.value = null;
  screenshotLoading.value = false;
});
</script>
