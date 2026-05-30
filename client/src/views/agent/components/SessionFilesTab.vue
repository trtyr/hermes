<template>
  <div class="flex flex-col min-h-0 flex-1 overflow-y-auto">
    <!-- Path bar -->
    <div class="flex items-center gap-2 px-3 py-2 border-b border-gray-100">
      <a-input
        v-model:value="browsePathInput"
        placeholder="输入路径，如 C:\ 或 /home"
        class="flex-1"
        size="small"
        @pressEnter="browseFromInput"
      >
        <template #prefix>
          <FolderOpenOutlined class="text-slate-400" />
        </template>
      </a-input>
      <a-button size="small" type="primary" :loading="browseLoading" @click="browseFromInput">
        浏览
      </a-button>
      <a-button size="small" :loading="browseLoading" @click="refreshCurrentPath">
        <template #icon><ReloadOutlined /></template>
      </a-button>
      <a-button size="small" @click="fileOpsVisible = true">
        <template #icon><UploadOutlined /></template>
        上传
      </a-button>
    </div>

    <!-- File list -->
    <div class="flex-1 overflow-y-auto min-h-0">
      <a-spin :spinning="browseLoading">
        <template v-if="browseError">
          <div class="flex flex-col items-center justify-center h-full min-h-[200px] text-slate-400">
            <ExclamationCircleOutlined style="font-size: 36px; opacity: 0.4; color: #faad14" />
            <p class="mt-3 text-sm text-amber-500">{{ browseError }}</p>
            <a-button size="small" class="mt-2" @click="browseFromInput">重试</a-button>
          </div>
        </template>
        <template v-else-if="browseEntries.length === 0 && !browseLoading && currentBrowsePath">
          <div class="flex flex-col items-center justify-center h-full min-h-[200px] text-slate-400">
            <FolderOpenOutlined style="font-size: 36px; opacity: 0.3" />
            <p class="mt-3 text-sm">目录为空</p>
          </div>
        </template>
        <template v-else-if="browseEntries.length > 0">
          <table class="w-full text-sm">
            <thead>
              <tr class="border-b border-gray-100 text-left text-slate-500 text-xs">
                <th class="py-2 px-3 font-medium">名称</th>
                <th class="py-2 px-3 font-medium w-28">大小</th>
                <th class="py-2 px-3 font-medium w-44">修改时间</th>
                <th class="py-2 px-3 font-medium w-20">操作</th>
              </tr>
            </thead>
            <tbody>
              <!-- Parent directory entry -->
              <tr
                v-if="currentBrowsePath"
                class="hover:bg-gray-50 cursor-pointer border-b border-gray-50"
                @click="browseParent"
              >
                <td class="py-1.5 px-3 flex items-center gap-2 text-slate-500">
                  <FolderOutlined class="text-amber-400" />
                  <span>..</span>
                </td>
                <td class="py-1.5 px-3 text-slate-400">-</td>
                <td class="py-1.5 px-3 text-slate-400">-</td>
                <td class="py-1.5 px-3">-</td>
              </tr>
              <!-- Directory entries -->
              <tr
                v-for="entry in browseEntries.filter(e => e.is_dir)"
                :key="'d-' + entry.name"
                class="hover:bg-gray-50 cursor-pointer border-b border-gray-50"
                @click="browseChild(entry.name)"
              >
                <td class="py-1.5 px-3 flex items-center gap-2">
                  <FolderOutlined class="text-amber-400" />
                  <span class="text-slate-700">{{ entry.name }}</span>
                </td>
                <td class="py-1.5 px-3 text-slate-400">-</td>
                <td class="py-1.5 px-3 text-slate-400">{{ formatTimestamp(entry.modified) }}</td>
                <td class="py-1.5 px-3">-</td>
              </tr>
              <!-- File entries -->
              <tr
                v-for="entry in browseEntries.filter(e => !e.is_dir)"
                :key="'f-' + entry.name"
                class="hover:bg-gray-50 border-b border-gray-50"
              >
                <td class="py-1.5 px-3 flex items-center gap-2">
                  <FileOutlined class="text-blue-400" />
                  <span class="text-slate-700">{{ entry.name }}</span>
                </td>
                <td class="py-1.5 px-3 text-slate-500">{{ formatSize(entry.size) }}</td>
                <td class="py-1.5 px-3 text-slate-500">{{ formatTimestamp(entry.modified) }}</td>
                <td class="py-1.5 px-3">
                  <a-button type="link" size="small" @click="onDownloadFile(entry.name)">
                    <DownloadOutlined />
                  </a-button>
                </td>
              </tr>
            </tbody>
          </table>
        </template>
        <template v-else>
          <div class="flex flex-col items-center justify-center h-full min-h-[200px] text-slate-400">
            <FolderOpenOutlined style="font-size: 36px; opacity: 0.3" />
            <p class="mt-3 text-sm">输入路径开始浏览文件</p>
          </div>
        </template>
      </a-spin>
    </div>

    <FileOpsModal v-model:visible="fileOpsVisible" :agent="agent" />
  </div>
</template>

<script setup lang="ts">
import { ref, onBeforeUnmount } from 'vue';
import { message } from 'ant-design-vue';
import {
  FolderOpenOutlined, FolderOutlined, FileOutlined,
  DownloadOutlined, ExclamationCircleOutlined, UploadOutlined,
  ReloadOutlined,
} from '@ant-design/icons-vue';
import { browseFile, downloadFile } from '@/api/agent';
import type { Agent, FileEntry } from '@/api/agent';
import { useEventStore } from '@/store/events';
import FileOpsModal from './FileOpsModal.vue';

const props = defineProps<{ agentId: string; agent: Agent | null }>();

// File browser state
const currentBrowsePath = ref('');
const browsePathInput = ref('');
const browseEntries = ref<FileEntry[]>([]);
const browseLoading = ref(false);
const browseError = ref<string | null>(null);
const pendingBrowseTaskId = ref<string | null>(null);
const fileOpsVisible = ref(false);
const browseCache = ref<Record<string, FileEntry[]>>({});
let unsubscribeBrowseEvents: (() => void) | null = null;

const eventStore = useEventStore();

// Auto-browse root when mounted for the first time
function initBrowse() {
  if (!currentBrowsePath.value) {
    doBrowse('C:\\');
  }
}

async function doBrowse(path: string, forceRefresh = false) {
  currentBrowsePath.value = path;
  browsePathInput.value = path;
  browseError.value = null;

  // Check cache first (unless force refresh)
  if (!forceRefresh && browseCache.value[path]) {
    browseEntries.value = browseCache.value[path];
    browseLoading.value = false;
    return;
  }

  browseLoading.value = true;
  browseEntries.value = [];
  pendingBrowseTaskId.value = null;

  if (unsubscribeBrowseEvents) {
    unsubscribeBrowseEvents();
    unsubscribeBrowseEvents = null;
  }

  try {
    const res = await browseFile(props.agentId, path);
    if (res.success && res.task_id) {
      pendingBrowseTaskId.value = res.task_id;

      unsubscribeBrowseEvents = eventStore.subscribe((event) => {
        if (event.type !== 'task_result') return;
        if (event.task_id !== pendingBrowseTaskId.value) return;

        const { success, output } = event;

        browseLoading.value = false;
        pendingBrowseTaskId.value = null;

        if (success && output) {
          try {
            const entries = JSON.parse(output) as FileEntry[];
            browseEntries.value = entries;
            browseCache.value[path] = entries;
          } catch {
            browseError.value = '解析目录列表失败';
          }
        } else {
          browseError.value = output || '浏览目录失败';
        }

        if (unsubscribeBrowseEvents) {
          unsubscribeBrowseEvents();
          unsubscribeBrowseEvents = null;
        }
      });
    } else {
      browseLoading.value = false;
      browseError.value = res.detail || '浏览任务下发失败';
    }
  } catch (e: any) {
    browseLoading.value = false;
    browseError.value = e.message;
  }
}

function browseFromInput() {
  const path = browsePathInput.value.trim();
  if (!path) return;
  doBrowse(path);
}

function refreshCurrentPath() {
  if (currentBrowsePath.value) {
    doBrowse(currentBrowsePath.value, true);
  }
}

function browseChild(name: string) {
  const sep = currentBrowsePath.value.includes('/') ? '/' : '\\';
  const base = currentBrowsePath.value.endsWith(sep) ? currentBrowsePath.value : currentBrowsePath.value + sep;
  doBrowse(base + name);
}

function browseParent() {
  const path = currentBrowsePath.value;
  if (!path) return;

  const sep = path.includes('/') ? '/' : '\\';
  const parts = path.replace(/[\\/]+$/, '').split(sep).filter(Boolean);
  if (parts.length <= 1) {
    if (sep === '\\') {
      doBrowse(parts[0] + '\\');
    } else {
      doBrowse('/');
    }
    return;
  }
  parts.pop();
  doBrowse(parts.join(sep) + sep);
}

function onDownloadFile(fileName: string) {
  const sep = currentBrowsePath.value.includes('/') ? '/' : '\\';
  const base = currentBrowsePath.value.endsWith(sep) ? currentBrowsePath.value : currentBrowsePath.value + sep;
  const remotePath = base + fileName;
  downloadFile(props.agentId, remotePath)
    .then((res) => {
      if (res.task_id) {
        eventStore.registerDownload(res.task_id, fileName);
      }
      message.info(`下载任务已下发: ${fileName}`);
    })
    .catch((e: any) => message.error(e.message));
}

function formatSize(bytes: number): string {
  if (bytes === 0) return '0 B';
  const units = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(1024));
  const val = bytes / Math.pow(1024, i);
  return `${val.toFixed(i === 0 ? 0 : 1)} ${units[i]}`;
}

function formatTimestamp(ts: number): string {
  if (!ts || ts === 0) return '-';
  const d = new Date(ts * 1000);
  const pad = (n: number) => n.toString().padStart(2, '0');
  return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())} ${pad(d.getHours())}:${pad(d.getMinutes())}:${pad(d.getSeconds())}`;
}

onBeforeUnmount(() => {
  if (unsubscribeBrowseEvents) {
    unsubscribeBrowseEvents();
    unsubscribeBrowseEvents = null;
  }
  pendingBrowseTaskId.value = null;
  browseLoading.value = false;
});

defineExpose({ initBrowse });
</script>
