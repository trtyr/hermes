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
          <WindowsOutlined v-if="agent.os && agent.os.toLowerCase().includes('windows')" class="text-blue-500" />
          <AppleOutlined v-else-if="agent.os && (agent.os.toLowerCase().includes('mac') || agent.os.toLowerCase().includes('darwin'))" class="text-gray-500" />
          <DesktopOutlined v-else class="text-slate-500" />
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

        <a-tab-pane key="files" tab="文件">
          <div class="flex flex-col items-center justify-center h-full min-h-[300px] text-slate-400">
            <FolderOpenOutlined style="font-size: 48px; opacity: 0.3" />
            <p class="mt-4 text-sm">文件管理功能开发中...</p>
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
import { ref, reactive, onMounted, computed } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { message, Modal } from 'ant-design-vue';
import {
  LeftOutlined, WindowsOutlined, AppleOutlined, DesktopOutlined,
  SafetyCertificateOutlined, SettingOutlined, CameraOutlined,
  FolderOpenOutlined, ClockCircleOutlined,
  DisconnectOutlined, DeleteOutlined
} from '@ant-design/icons-vue';
import {
  fetchAgentDetail, disconnectAgent,
  deleteAgent, takeScreenshot, updateBeaconConfig
} from '@/api/agent';
import type { Agent } from '@/api/agent';
import { useTerminal } from './hooks/useTerminal';
import { useAppStore } from '@/store/app';
import 'xterm/css/xterm.css';

const route = useRoute();
const router = useRouter();
const appStore = useAppStore();
const agentId = route.params.id as string;

// Agent data
const agent = ref<Agent | null>(null);
const activeTab = ref('terminal');

// Screenshot state
const screenshotLoading = ref(false);
const screenshotUrl = ref<string | null>(null);

// Beacon config state
const beaconModalVisible = ref(false);
const beaconSubmitting = ref(false);
const beaconForm = reactive({ sleep_interval: 30, jitter: 20 });

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

async function doScreenshot() {
  screenshotLoading.value = true;
  screenshotUrl.value = null;
  try {
    const res = await takeScreenshot(agentId);
    if (res.success) {
      message.success(`截图任务已下发 (task: ${res.task_id})`);
    } else {
      message.error(res.detail || '截图失败');
    }
  } catch (e: any) {
    message.error(e.message);
  } finally {
    screenshotLoading.value = false;
  }
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
</script>

<style>
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
