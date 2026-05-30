<template>
  <div class="flex flex-col h-full">
    <!-- Top bar -->
    <div class="flex items-center gap-2 px-4 py-3 border-b border-gray-100 shrink-0">
      <a-button type="primary" :disabled="!isOnline" :loading="screenshotLoading" @click="doScreenshot">
        <template #icon><CameraOutlined /></template>
        截取屏幕
      </a-button>
      <a-button :loading="galleryLoading" @click="fetchGallery">
        <template #icon><ReloadOutlined /></template>
        刷新
      </a-button>
      <span v-if="screenshots.length" class="ml-2 text-xs text-slate-400">共 {{ screenshots.length }} 张</span>
    </div>

    <!-- Content -->
    <div class="flex-1 overflow-auto p-4">
      <!-- Gallery loading -->
      <div v-if="galleryLoading && !screenshots.length" class="flex items-center justify-center h-full">
        <a-spin size="large" />
      </div>

      <!-- Empty state -->
      <div v-else-if="!screenshots.length" class="flex flex-col items-center justify-center h-full text-slate-400">
        <CameraOutlined style="font-size: 48px; opacity: 0.3" />
        <p class="mt-4 text-sm">暂无截图记录</p>
        <p class="text-xs mt-1">点击「截取屏幕」按钮开始</p>
      </div>

      <!-- Screenshot taking overlay -->
      <div v-else-if="screenshotLoading" class="flex flex-col items-center justify-center py-12">
        <a-spin size="large" />
        <p class="mt-4 text-sm text-slate-400">正在截图...</p>
      </div>

      <!-- Gallery grid -->
      <div v-else class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-3">
        <div
          v-for="(s, idx) in screenshots"
          :key="s.task_id"
          role="button"
          tabindex="0"
          class="group relative cursor-pointer rounded-lg overflow-hidden border border-gray-200 bg-gray-50 hover:border-blue-400 hover:shadow-md transition-all focus:outline-none focus:ring-2 focus:ring-blue-400"
          @click="openLightbox(idx)"
          @keydown.enter="openLightbox(idx)"
          @keydown.space.prevent="openLightbox(idx)"
        >
          <img
            :src="s.dataUrl"
            :alt="`截图 ${formatTime(s.created_at)}`"
            class="w-full aspect-video object-cover"
          />
          <!-- Timestamp overlay -->
          <div class="absolute bottom-0 inset-x-0 bg-gradient-to-t from-black/60 to-transparent px-2 py-1.5">
            <span class="text-[11px] text-white/90">{{ formatTime(s.created_at) }}</span>
          </div>
          <!-- Hover overlay -->
          <div class="absolute inset-0 bg-black/0 group-hover:bg-black/10 transition-colors flex items-center justify-center">
            <EyeOutlined class="text-white text-xl opacity-0 group-hover:opacity-100 transition-opacity drop-shadow" />
          </div>
        </div>
      </div>
    </div>

    <!-- Lightbox modal -->
    <a-modal
      v-model:open="lightboxVisible"
      :footer="null"
      :width="800"
      :body-style="{ padding: 0 }"
      centered
      @keydown.left="prevScreenshot"
      @keydown.right="nextScreenshot"
    >
      <div v-if="currentScreenshot" class="relative">
        <!-- Full-size image -->
        <img
          :src="currentScreenshot.dataUrl"
          :alt="`截图 ${formatTime(currentScreenshot.created_at)}`"
          class="w-full"
        />
        <!-- Info bar -->
        <div class="absolute bottom-0 inset-x-0 bg-gradient-to-t from-black/70 to-transparent px-4 py-3 flex items-end justify-between">
          <div>
            <div class="text-white text-sm font-medium">{{ formatTime(currentScreenshot.created_at) }}</div>
            <div class="text-white/60 text-xs mt-0.5">Task: {{ currentScreenshot.task_id }}</div>
          </div>
          <div class="text-white/60 text-xs">{{ lightboxIndex + 1 }} / {{ screenshots.length }}</div>
        </div>
        <!-- Prev / Next buttons -->
        <a-button
          v-if="screenshots.length > 1"
          class="absolute left-2 top-1/2 -translate-y-1/2"
          shape="circle"
          :disabled="lightboxIndex === 0"
          @click.stop="prevScreenshot"
        >
          <template #icon><LeftOutlined /></template>
        </a-button>
        <a-button
          v-if="screenshots.length > 1"
          class="absolute right-2 top-1/2 -translate-y-1/2"
          shape="circle"
          :disabled="lightboxIndex === screenshots.length - 1"
          @click.stop="nextScreenshot"
        >
          <template #icon><RightOutlined /></template>
        </a-button>
      </div>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount } from 'vue';
import { message } from 'ant-design-vue';
import { CameraOutlined, ReloadOutlined, EyeOutlined, LeftOutlined, RightOutlined } from '@ant-design/icons-vue';
import { takeScreenshot, listTasks } from '@/api/agent';
import { useEventStore } from '@/store/events';
import dayjs from 'dayjs';

const props = defineProps<{ agentId: string; isOnline: boolean }>();

// ─── Gallery state ──────────────────────────────────────────────────────────

interface Screenshot {
  task_id: string;
  created_at: number;  // Unix seconds
  dataUrl: string;     // data:image/png;base64,...
}

const screenshots = ref<Screenshot[]>([]);
const galleryLoading = ref(false);

async function fetchGallery() {
  galleryLoading.value = true;
  try {
    const res = await listTasks({ agent_id: props.agentId, command: 'screenshot' });
    const items: Screenshot[] = [];
    for (const t of res.tasks) {
      if (t.success && t.output) {
        items.push({
          task_id: t.task_id,
          created_at: t.created_at,
          dataUrl: `data:image/png;base64,${t.output}`,
        });
      }
    }
    // Sort newest first
    items.sort((a, b) => b.created_at - a.created_at);
    screenshots.value = items;
  } catch (e: any) {
    message.error(e.message || '加载截图列表失败');
  } finally {
    galleryLoading.value = false;
  }
}

// ─── Screenshot trigger ─────────────────────────────────────────────────────

const screenshotLoading = ref(false);
const pendingScreenshotTaskId = ref<string | null>(null);
let unsubscribeEvents: (() => void) | null = null;
let screenshotTimeoutId: ReturnType<typeof setTimeout> | null = null;

const eventStore = useEventStore();

async function doScreenshot() {
  screenshotLoading.value = true;
  pendingScreenshotTaskId.value = null;

  // Cleanup previous pending state
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

      // Register globally — result survives page navigation
      eventStore.registerPendingTask(res.task_id, 'screenshot');

      // Check if result already arrived (unlikely but possible)
      const cached = eventStore.getTaskResult(res.task_id);
      if (cached) {
        screenshotLoading.value = false;
        pendingScreenshotTaskId.value = null;
        eventStore.clearPendingTask(res.task_id);
        if (cached.success) {
          message.success('截图成功');
          fetchGallery();
        } else {
          message.error(cached.output || '截图失败');
        }
        return;
      }

      unsubscribeEvents = eventStore.subscribe((event) => {
        if (event.type !== 'task_result') return;
        if (event.task_id !== pendingScreenshotTaskId.value) return;

        if (screenshotTimeoutId) {
          clearTimeout(screenshotTimeoutId);
          screenshotTimeoutId = null;
        }

        screenshotLoading.value = false;
        pendingScreenshotTaskId.value = null;
        eventStore.clearPendingTask(event.task_id);

        const { success, output } = event;

        if (success && output) {
          message.success('截图成功');
          // Refresh gallery to include the new screenshot
          fetchGallery();
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

// ─── Lightbox ───────────────────────────────────────────────────────────────

const lightboxVisible = ref(false);
const lightboxIndex = ref(0);

const currentScreenshot = computed(() => screenshots.value[lightboxIndex.value] ?? null);

function openLightbox(idx: number) {
  lightboxIndex.value = idx;
  lightboxVisible.value = true;
}

function prevScreenshot() {
  if (lightboxIndex.value > 0) lightboxIndex.value--;
}

function nextScreenshot() {
  if (lightboxIndex.value < screenshots.value.length - 1) lightboxIndex.value++;
}

// ─── Helpers ────────────────────────────────────────────────────────────────

function formatTime(ts: number): string {
  return dayjs.unix(ts).format('YYYY-MM-DD HH:mm:ss');
}

// ─── Lifecycle ──────────────────────────────────────────────────────────────

onMounted(() => {
  fetchGallery();
  // Check if a pending screenshot completed while we were unmounted
  if (pendingScreenshotTaskId.value) {
    const cached = eventStore.getTaskResult(pendingScreenshotTaskId.value);
    if (cached) {
      screenshotLoading.value = false;
      pendingScreenshotTaskId.value = null;
      eventStore.clearPendingTask(cached.task_id);
      if (cached.success) {
        message.success('截图成功');
        fetchGallery();
      } else {
        message.error(cached.output || '截图失败');
      }
    }
  }
});

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
