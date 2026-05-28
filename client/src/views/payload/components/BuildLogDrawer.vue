<template>
  <a-drawer
    :open="visible"
    :title="`构建日志 #${buildId}`"
    :width="700"
    @close="handleClose"
  >
    <template #extra>
      <a-button size="small" @click="fetchBuildLog">
        <template #icon><ReloadOutlined /></template>
        刷新
      </a-button>
    </template>
    <div
      ref="logContainerRef"
      class="bg-gray-900 text-green-400 p-4 rounded text-xs font-mono whitespace-pre-wrap break-all"
      style="max-height: calc(100vh - 200px); overflow-y: auto"
    >
      <template v-if="logContent">{{ logContent }}</template>
      <span v-else class="text-gray-500">暂无日志输出...</span>
    </div>
    <div v-if="buildStatus === 'pending'" class="mt-3 text-sm text-slate-500 flex items-center gap-2">
      <span class="relative flex h-2.5 w-2.5 shrink-0">
        <span class="absolute inline-flex h-full w-full animate-ping rounded-full bg-blue-400 opacity-75"></span>
        <span class="relative inline-flex h-2.5 w-2.5 rounded-full bg-blue-400"></span>
      </span>
      构建进行中，日志自动刷新中...
    </div>
  </a-drawer>
</template>

<script setup lang="ts">
import { ref, watch, onUnmounted, nextTick } from 'vue';
import { ReloadOutlined } from '@ant-design/icons-vue';
import { fetchAgentBuild } from '@/api/agentBuild';

const props = defineProps<{
  visible: boolean;
  buildId: number | null;
  initialStatus: string;
  initialDetail: string;
}>();

const emit = defineEmits<{
  (e: 'update:visible', val: boolean): void;
  (e: 'completed'): void;
}>();

const buildStatus = ref('');
const logContent = ref('');
let pollTimer: ReturnType<typeof setInterval> | null = null;
const logContainerRef = ref<HTMLElement | null>(null);

watch(() => props.visible, (isOpen) => {
  if (isOpen && props.buildId) {
    buildStatus.value = props.initialStatus;
    logContent.value = props.initialDetail || '';
    fetchBuildLog();
    if (props.initialStatus === 'pending') {
      startPolling();
    }
  } else {
    stopPolling();
  }
});

onUnmounted(() => {
  stopPolling();
});

function handleClose() {
  emit('update:visible', false);
  stopPolling();
}

async function fetchBuildLog() {
  if (!props.buildId) return;
  try {
    const build = await fetchAgentBuild(props.buildId);
    logContent.value = build.detail || '';
    buildStatus.value = build.status;
    if (build.status !== 'pending') {
      stopPolling();
      emit('completed');
    }
    await nextTick();
    scrollToBottom();
  } catch {
    // Silently fail — will retry on next poll
  }
}

function startPolling() {
  stopPolling();
  pollTimer = setInterval(() => {
    fetchBuildLog();
  }, 2000);
}

function stopPolling() {
  if (pollTimer) {
    clearInterval(pollTimer);
    pollTimer = null;
  }
}

function scrollToBottom() {
  if (logContainerRef.value) {
    logContainerRef.value.scrollTop = logContainerRef.value.scrollHeight;
  }
}
</script>
