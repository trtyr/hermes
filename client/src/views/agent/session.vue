<template>
  <div class="h-full w-full flex flex-col bg-[#f0f2f5]">
    <!-- Top info bar -->
    <div class="bg-white rounded-lg border border-gray-200 shadow-sm px-4 py-2.5 mb-3 flex items-center gap-4">
      <!-- Back button -->
      <a-button type="text" size="small" @click="router.push('/agent')" class="flex items-center gap-1 text-slate-500 hover:text-slate-800">
        <LeftOutlined />
        <span>返回</span>
      </a-button>

      <div class="h-4 w-px bg-gray-200"></div>

      <!-- Agent info -->
      <template v-if="agent">
        <!-- hostname \ username -->
        <div class="flex items-center gap-1.5 font-semibold text-slate-800">
          <span>{{ agent.hostname || agent.agent_id }}</span>
          <span class="text-slate-400 font-normal">\</span>
          <span class="text-slate-600 font-normal">{{ agent.username || '-' }}</span>
        </div>

        <div class="h-4 w-px bg-gray-200"></div>

        <!-- OS + arch -->
        <div class="flex items-center gap-1.5 text-sm text-slate-600">
          <WindowsOutlined class="text-blue-500" />
          <span>{{ agent.os }} {{ agent.arch }}</span>
        </div>

        <div class="h-4 w-px bg-gray-200"></div>

        <!-- Privilege shield -->
        <a-tooltip v-if="(agent as any).privilege" :title="(agent as any).privilege">
          <SafetyCertificateOutlined
            :style="{
              color: (agent as any).privilege?.startsWith('Admin') || (agent as any).privilege === 'SYSTEM' ? '#f5222d' : '#8c8c8c',
              fontSize: '16px'
            }"
          />
        </a-tooltip>

        <div class="h-4 w-px bg-gray-200"></div>

        <!-- Online / Offline badge -->
        <div class="flex items-center gap-1.5">
          <span
            class="inline-block w-2 h-2 rounded-full"
            :class="agent.is_online ? 'bg-green-500' : 'bg-gray-300'"
          ></span>
          <span class="text-sm" :class="agent.is_online ? 'text-green-600' : 'text-slate-400'">
            {{ agent.is_online ? '在线' : '离线' }}
          </span>
        </div>

        <!-- Spacer -->
        <div class="flex-1"></div>

        <!-- Secondary info (smaller text) -->
        <div class="text-xs text-slate-400 flex items-center gap-3">
          <span v-if="agent.listener_name">📡 {{ agent.listener_name }}</span>
          <span v-if="agent.internal_ip">{{ agent.internal_ip }}</span>
          <span v-if="agent.pid">PID:{{ agent.pid }}</span>
          <span>{{ agent.sleep_interval }}s±{{ agent.jitter }}%</span>
        </div>
      </template>

      <div v-else class="text-slate-400 text-sm">加载中...</div>
    </div>

    <!-- Tab navigation + content -->
    <div class="flex-1 flex flex-col bg-white rounded-lg border border-gray-200 shadow-sm overflow-hidden">
      <a-tabs
        v-model:activeKey="activeTab"
        class="session-tabs h-full flex flex-col"
        :class="{ 'terminal-active': activeTab === 'terminal' }"
      >
        <a-tab-pane key="terminal" tab="终端" force-render class="h-full">
          <div class="h-full bg-[#1e1e1e] relative">
            <div ref="terminalContainer" class="absolute inset-0 p-3 pt-2"></div>
          </div>
        </a-tab-pane>

        <a-tab-pane key="files" tab="文件" force-render class="h-full">
          <div class="flex flex-col h-full">
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
            </div>

            <!-- File list -->
            <div class="flex-1 overflow-auto">
              <a-spin :spinning="browseLoading" class="h-full">
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
          </div>
        </a-tab-pane>

        <a-tab-pane key="screenshot" tab="截屏">
          <div class="flex flex-col items-center justify-center h-full min-h-[300px]">
            <template v-if="screenshotLoading">
              <a-spin size="large" />
              <p class="mt-4 text-sm text-slate-400">正在截图...</p>
            </template>
            <template v-else-if="screenshotUrl">
              <img :src="screenshotUrl" class="max-w-full max-h-[60vh] rounded border border-gray-200 shadow-sm" />
              <a-button class="mt-4" @click="doScreenshot">
                <template #icon><CameraOutlined /></template>
                重新截图
              </a-button>
            </template>
            <template v-else>
              <CameraOutlined style="font-size: 48px; opacity: 0.3; color: #8c8c8c" />
              <p class="mt-4 text-sm text-slate-400">点击下方按钮截取目标屏幕</p>
              <a-button type="primary" class="mt-2" :disabled="!agent?.is_online" @click="doScreenshot">
                <template #icon><CameraOutlined /></template>
                截取屏幕
              </a-button>
            </template>
          </div>
        </a-tab-pane>

        <a-tab-pane key="proxy" tab="代理" force-render class="h-full">
          <div class="flex flex-col h-full p-4 overflow-auto">
            <!-- Start proxy button -->
            <div class="flex items-center justify-between mb-4">
              <span class="text-sm text-slate-600">SOCKS5 代理隧道</span>
              <a-button
                type="primary"
                size="small"
                :loading="proxyStarting"
                @click="handleStartProxy"
              >
                <template #icon><PlusOutlined /></template>
                新建代理
              </a-button>
            </div>

            <!-- Empty state -->
            <div v-if="proxies.length === 0" class="flex flex-col items-center justify-center flex-1 text-slate-400">
              <ApiOutlined style="font-size: 36px; opacity: 0.3" />
              <p class="mt-3 text-sm">暂无代理会话</p>
              <p class="text-xs text-slate-300 mt-1">点击“新建代理”开启 SOCKS5 隧道</p>
            </div>

            <!-- Proxy cards -->
            <div v-else class="space-y-3">
              <div
                v-for="p in proxies"
                :key="p.proxy_id"
                class="border border-gray-200 rounded-lg p-3 hover:border-blue-200 transition-colors"
              >
                <div class="flex items-center justify-between mb-2">
                  <div class="flex items-center gap-2">
                    <span class="inline-block w-2 h-2 rounded-full" :class="p.status === 'open' ? 'bg-green-500' : 'bg-gray-300'"></span>
                    <span class="font-mono text-sm text-slate-700">{{ p.proxy_id }}</span>
                    <a-tag :color="p.status === 'open' ? 'green' : 'default'" size="small" class="mr-0">
                      {{ p.status === 'open' ? '运行中' : p.status }}
                    </a-tag>
                  </div>
                  <a-button
                    type="link"
                    size="small"
                    :loading="proxyStopping === p.proxy_id"
                    @click="handleStopProxy(p.proxy_id)"
                  >
                    停止
                  </a-button>
                </div>

                <div class="text-xs text-slate-500 space-y-1">
                  <div class="flex items-center gap-2">
                    <span class="text-slate-400">绑定地址:</span>
                    <code class="bg-gray-100 px-1.5 py-0.5 rounded font-mono text-blue-600">{{ p.bind_addr }}</code>
                    <a-button
                      type="link"
                      size="small"
                      class="!p-0 !h-auto !text-xs"
                      @click="copyProxyAddr(p.bind_addr)"
                    >
                      <template #icon><CopyOutlined /></template>
                    </a-button>
                  </div>
                  <div>
                    <span class="text-slate-400">活跃流:</span>
                    <span class="ml-1">{{ p.active_streams }}</span>
                  </div>
                  <div v-if="p.last_error" class="text-red-500">
                    <span class="text-slate-400">错误:</span>
                    <span class="ml-1">{{ p.last_error }}</span>
                  </div>
                </div>
              </div>
            </div>

            <!-- Closed proxies section (reference only) -->
            <div v-if="closedProxies.length > 0" class="mt-4">
              <div class="text-xs text-slate-400 mb-2">已停止的代理 ({{ closedProxies.length }})</div>
              <div
                v-for="p in closedProxies"
                :key="'closed-' + p.proxy_id"
                class="border border-gray-100 rounded p-2 mb-2 opacity-60"
              >
                <div class="flex items-center justify-between">
                  <div>
                    <span class="font-mono text-xs text-slate-500">{{ p.proxy_id }}</span>
                    <span class="text-xs text-slate-400 ml-2">{{ p.bind_addr }}</span>
                  </div>
                </div>
              </div>
            </div>

            <!-- Help text -->
            <div v-if="proxies.length > 0" class="mt-3 p-2 bg-gray-50 rounded text-xs text-slate-500">
              <p class="font-medium mb-1">使用方法:</p>
              <code class="block">curl --socks5 {{ proxies[0]?.bind_addr || '127.0.0.1:PORT' }} http://内网地址</code>
              <p class="mt-1 text-slate-400">支持标准 SOCKS5 协议的客户端均可使用</p>
            </div>
          </div>
        </a-tab-pane>

        <!-- Dropdown menu rendered as extra tab content -->
        <template #rightExtra>
          <a-dropdown :trigger="['click']">
            <a-button type="text" size="small" class="mr-2">
              <SettingOutlined />
            </a-button>
            <template #overlay>
              <a-menu @click="onMenuClick">
                <a-menu-item key="beacon">
                  <template #icon><ClockCircleOutlined /></template>
                  调整心跳
                </a-menu-item>
                <a-menu-divider />
                <a-menu-item key="disconnect" :disabled="!agent?.is_online">
                  <template #icon><DisconnectOutlined /></template>
                  断开连接
                </a-menu-item>
                <a-menu-item key="delete" danger>
                  <template #icon><DeleteOutlined /></template>
                  删除 Agent
                </a-menu-item>
              </a-menu>
            </template>
          </a-dropdown>
        </template>
      </a-tabs>
    </div>

    <!-- Beacon config modal -->
    <a-modal
      v-model:open="beaconModalVisible"
      title="调整心跳配置"
      @ok="submitBeaconConfig"
      :confirm-loading="beaconSubmitting"
    >
      <div class="py-4 space-y-4">
        <div>
          <label class="block text-sm text-slate-600 mb-1">Sleep Interval (秒)</label>
          <a-input-number v-model:value="beaconForm.sleep_interval" :min="1" :max="3600" class="w-full" />
        </div>
        <div>
          <label class="block text-sm text-slate-600 mb-1">Jitter (%)</label>
          <a-input-number v-model:value="beaconForm.jitter" :min="0" :max="100" class="w-full" />
        </div>
      </div>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, onBeforeUnmount, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { message, Modal } from 'ant-design-vue';
import {
  LeftOutlined, WindowsOutlined,
  SafetyCertificateOutlined, SettingOutlined, CameraOutlined,
  FolderOpenOutlined, FolderOutlined, FileOutlined, DownloadOutlined,
  ExclamationCircleOutlined, ClockCircleOutlined,
  DisconnectOutlined, DeleteOutlined,
  ApiOutlined, CopyOutlined, PlusOutlined
} from '@ant-design/icons-vue';
import {
  fetchAgentDetail, disconnectAgent,
  deleteAgent, takeScreenshot, updateBeaconConfig, browseFile, downloadFile
} from '@/api/agent';
import type { Agent, FileEntry } from '@/api/agent';
import { listProxies, startProxy as apiStartProxy, deleteProxy as apiDeleteProxy } from '@/api/proxy';
import type { ProxySessionRecord } from '@/api/proxy';
import { useTerminal } from './hooks/useTerminal';
import { useAppStore } from '@/store/app';
import { useEventStore } from '@/store/events';
import 'xterm/css/xterm.css';

const route = useRoute();
const router = useRouter();
const appStore = useAppStore();
const eventStore = useEventStore();
const agentId = route.params.id as string;

// Agent data
const agent = ref<Agent | null>(null);
const activeTab = ref('terminal');

// Screenshot state
const screenshotLoading = ref(false);
const screenshotUrl = ref<string | null>(null);
const pendingScreenshotTaskId = ref<string | null>(null);
let unsubscribeEvents: (() => void) | null = null;
let screenshotTimeoutId: ReturnType<typeof setTimeout> | null = null;

// File browser state
const currentBrowsePath = ref<string>('');
const browsePathInput = ref<string>('');
const browseEntries = ref<FileEntry[]>([]);
const browseLoading = ref(false);
const browseError = ref<string | null>(null);
const pendingBrowseTaskId = ref<string | null>(null);
let unsubscribeBrowseEvents: (() => void) | null = null;

// Auto-browse root directory when files tab is first opened
watch(activeTab, (tab) => {
  if (tab === 'files' && !currentBrowsePath.value) {
    doBrowse('C:\\');
  }
  if (tab === 'proxy') {
    loadProxies();
  }
});

// Beacon config state
const beaconModalVisible = ref(false);
const beaconSubmitting = ref(false);
const beaconForm = reactive({ sleep_interval: 30, jitter: 20 });

// Proxy state
const proxies = ref<ProxySessionRecord[]>([]);
const closedProxies = ref<ProxySessionRecord[]>([]);
const proxyStarting = ref(false);
const proxyStopping = ref<string | null>(null);

// Terminal hook
const { terminalContainer, sessionId, wsConnected } = useTerminal(agentId);

// Sync tab title
onMounted(async () => {
  const currentView = appStore.visitedViews.find(v => v.path === route.path);
  if (currentView) {
    currentView.title = `会话: ${agentId}`;
  } else {
    appStore.addView({
      path: route.path,
      name: route.name as string || 'AgentSession',
      title: `会话: ${agentId}`
    });
  }

  // Load agent detail
  try {
    agent.value = await fetchAgentDetail(agentId);
    beaconForm.sleep_interval = agent.value.sleep_interval;
    beaconForm.jitter = agent.value.jitter;
  } catch (e: any) {
    message.error(e.message || '加载节点信息失败');
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
  if (unsubscribeBrowseEvents) {
    unsubscribeBrowseEvents();
    unsubscribeBrowseEvents = null;
  }
  pendingScreenshotTaskId.value = null;
  screenshotLoading.value = false;
  pendingBrowseTaskId.value = null;
  browseLoading.value = false;
});

async function doScreenshot() {
  screenshotLoading.value = true;
  screenshotUrl.value = null;
  pendingScreenshotTaskId.value = null;

  // Clean up any previous subscription and timeout
  if (screenshotTimeoutId) {
    clearTimeout(screenshotTimeoutId);
    screenshotTimeoutId = null;
  }
  if (unsubscribeEvents) {
    unsubscribeEvents();
    unsubscribeEvents = null;
  }

  try {
    const res = await takeScreenshot(agentId);
    if (res.success && res.task_id) {
      pendingScreenshotTaskId.value = res.task_id;

      // Subscribe to WebSocket events and wait for the task result
      unsubscribeEvents = eventStore.subscribe((event) => {
        if (event.type !== 'task_result') return;
        const taskId = (event as any).task_id as string;
        if (taskId !== pendingScreenshotTaskId.value) return;

        // Clear timeout on success
        if (screenshotTimeoutId) {
          clearTimeout(screenshotTimeoutId);
          screenshotTimeoutId = null;
        }

        screenshotLoading.value = false;
        pendingScreenshotTaskId.value = null;

        const success = (event as any).success as boolean;
        const output = (event as any).output as string;

        if (success && output) {
          screenshotUrl.value = `data:image/png;base64,${output}`;
        } else {
          message.error(output || '截图失败');
        }

        // Auto-cleanup this subscription
        if (unsubscribeEvents) {
          unsubscribeEvents();
          unsubscribeEvents = null;
        }
      });

      // Timeout: if no result received within 60s, unsubscribe and stop loading
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

async function doBrowse(path: string) {
  browseLoading.value = true;
  browseError.value = null;
  browseEntries.value = [];
  currentBrowsePath.value = path;
  browsePathInput.value = path;
  pendingBrowseTaskId.value = null;

  // Clean up any previous browse subscription
  if (unsubscribeBrowseEvents) {
    unsubscribeBrowseEvents();
    unsubscribeBrowseEvents = null;
  }

  try {
    const res = await browseFile(agentId, path);
    if (res.success && res.task_id) {
      pendingBrowseTaskId.value = res.task_id;

      unsubscribeBrowseEvents = eventStore.subscribe((event) => {
        if (event.type !== 'task_result') return;
        const taskId = (event as any).task_id as string;
        if (taskId !== pendingBrowseTaskId.value) return;

        const success = (event as any).success as boolean;
        const output = (event as any).output as string;

        browseLoading.value = false;
        pendingBrowseTaskId.value = null;

        if (success && output) {
          try {
            browseEntries.value = JSON.parse(output) as FileEntry[];
          } catch {
            browseError.value = '解析目录列表失败';
          }
        } else {
          browseError.value = output || '浏览目录失败';
        }

        // Auto-cleanup
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

function browseChild(name: string) {
  const sep = currentBrowsePath.value.includes('/') ? '/' : '\\';
  const base = currentBrowsePath.value.endsWith(sep) ? currentBrowsePath.value : currentBrowsePath.value + sep;
  doBrowse(base + name);
}

function browseParent() {
  const path = currentBrowsePath.value;
  if (!path) return;

  const sep = path.includes('/') ? '/' : '\\';
  // Handle root paths (e.g. "C:\" or "/")
  const parts = path.replace(/[\\/]+$/, '').split(sep).filter(Boolean);
  if (parts.length <= 1) {
    // Already at root or drive root, browse root again
    if (sep === '\\') {
      // Windows: go to drive root like "C:\"
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
  downloadFile(agentId, remotePath)
    .then(() => message.info(`下载任务已下发: ${fileName}`))
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

function onMenuClick({ key }: { key: string }) {
  if (key === 'beacon') {
    beaconModalVisible.value = true;
    return;
  }

  const actionMap: Record<string, { title: string; func: (id: string) => Promise<any> }> = {
    disconnect: { title: '断开连接', func: disconnectAgent },
    delete: { title: '删除 Agent', func: deleteAgent },
  };

  const target = actionMap[key];
  if (!target) return;

  Modal.confirm({
    title: `确认执行 [${target.title}] 操作？`,
    content: key === 'delete' ? '删除后将不可恢复。' : '',
    okType: key === 'delete' || key === 'disconnect' ? 'danger' : 'primary',
    async onOk() {
      try {
        await target.func(agentId);
        message.success(`操作 [${target.title}] 执行成功`);
        if (key === 'delete') {
          router.push('/agent');
        } else {
          agent.value = await fetchAgentDetail(agentId);
        }
      } catch (e: any) {
        message.error(e.message);
      }
    }
  });
}

async function submitBeaconConfig() {
  beaconSubmitting.value = true;
  try {
    await updateBeaconConfig(agentId, beaconForm.sleep_interval, beaconForm.jitter);
    message.success('心跳配置已更新');
    beaconModalVisible.value = false;
    agent.value = await fetchAgentDetail(agentId);
  } catch (e: any) {
    message.error(e.message);
  } finally {
    beaconSubmitting.value = false;
  }
}

// Proxy management
const loadProxies = async () => {
  if (!agent.value) return;
  try {
    const res = await listProxies(agent.value.agent_id);
    const all = res.proxies || [];
    proxies.value = all.filter(p => p.status === 'open');
    closedProxies.value = all.filter(p => p.status !== 'open');
  } catch {
    // silently fail
  }
};

const handleStartProxy = async () => {
  if (!agent.value) return;
  proxyStarting.value = true;
  try {
    const res = await apiStartProxy(agent.value.agent_id);
    message.success(`代理已启动: ${res.proxy.bind_addr}`);
    await loadProxies();
  } catch (e: any) {
    message.error(e.message || '启动代理失败');
  } finally {
    proxyStarting.value = false;
  }
};

const handleStopProxy = async (proxyId: string) => {
  if (!agent.value) return;
  proxyStopping.value = proxyId;
  try {
    await apiDeleteProxy(agent.value.agent_id, proxyId);
    message.success(`代理 ${proxyId} 已删除`);
    await loadProxies();
  } catch (e: any) {
    message.error(e.message || '停止代理失败');
  } finally {
    proxyStopping.value = null;
  }
};

const copyProxyAddr = (addr: string) => {
  navigator.clipboard.writeText(addr).then(() => {
    message.success(`已复制: ${addr}`);
  });
};
</script>

<style>
.session-tabs .ant-tabs-nav {
  padding-left: 12px;
}

.session-tabs .ant-tabs-content {
  flex: 1;
  display: flex;
  flex-direction: column;
}

.session-tabs .ant-tabs-tabpane {
  flex: 1;
  display: flex;
  flex-direction: column;
}

.session-tabs .ant-tabs-content-holder {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.session-tabs.terminal-active .ant-tabs-content-holder {
  background: #1e1e1e;
}

/* XTerm scrollbar */
.xterm .xterm-viewport {
  overflow-y: auto !important;
}
.xterm .xterm-viewport::-webkit-scrollbar {
  width: 8px;
}
.xterm .xterm-viewport::-webkit-scrollbar-track {
  background: #111;
}
.xterm .xterm-viewport::-webkit-scrollbar-thumb {
  background: #555;
  border-radius: 4px;
}
</style>
