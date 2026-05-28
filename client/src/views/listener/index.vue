<template>
  <div class="h-full w-full flex flex-col p-4 relative">
    <!-- Agent认证设置 -->
    <a-card
      title="Agent 认证"
      class="mb-4"
      :bordered="true"
      size="small"
    >
      <template #extra>
        <a-tooltip title="新 Agent 连接时的认证方式和令牌">
          <QuestionCircleOutlined class="text-slate-400" />
        </a-tooltip>
      </template>
      <div class="flex items-start gap-6">
        <div class="flex-1">
          <label class="text-xs text-slate-500 mb-1 block">认证令牌</label>
          <a-input-password
            v-model:value="authForm.agent_token"
            placeholder="留空则不认证"
            :disabled="authLoading"
            size="small"
            class="max-w-xs"
          />
        </div>
        <div class="flex-1">
          <label class="text-xs text-slate-500 mb-1 block">认证模式</label>
          <a-radio-group v-model:value="authForm.agent_auth_mode" :disabled="authLoading" size="small">
            <a-radio-button value="plain_token">共享令牌</a-radio-button>
            <a-radio-button value="challenge_response">挑战-响应</a-radio-button>
          </a-radio-group>
        </div>
        <div class="flex items-end gap-2" style="padding-top: 1px;">
          <a-button type="primary" size="small" :loading="authLoading" @click="saveAuthSettings">
            保存
          </a-button>
          <a-button size="small" :loading="authLoading" @click="loadAuthSettings">
            刷新
          </a-button>
        </div>
      </div>
      <div class="text-xs text-slate-400 mt-2">
        <template v-if="authForm.agent_auth_mode === 'plain_token'">
          Agent 注册时明文携带令牌，Server 对比验证。令牌需在生成 Agent 时一并嵌入。
        </template>
        <template v-else>
          Server 发送随机 nonce，Agent 用 HMAC-SHA256 签名响应，令牌不在网络上传输。需在生成 Agent 时嵌入相同令牌。
        </template>
      </div>
    </a-card>

    <!-- Header -->
    <div class="flex justify-between items-center mb-4">
      <h2 class="text-xl font-semibold text-slate-800 flex items-center gap-2 m-0">
        <ApiOutlined class="text-indigo-500" />
        监听器管理
      </h2>
      <div class="flex gap-2">
        <template v-if="selectedRowKeys.length > 0">
          <span class="text-sm text-slate-500">已选中 {{ selectedRowKeys.length }} 项</span>
          <a-button type="link" size="small" @click="selectedRowKeys = []">取消选择</a-button>
          <a-button class="text-green-600 border-green-300 hover:text-green-700 hover:border-green-400" :loading="batchStartLoading" @click="handleBatchStart">
            批量启动
          </a-button>
          <a-button class="text-amber-600 border-amber-300 hover:text-amber-700 hover:border-amber-400" :loading="batchStopLoading" @click="handleBatchStop">
            批量停止
          </a-button>
          <a-button danger :loading="batchDeleteLoading" @click="handleBatchDelete">
            批量删除
          </a-button>
          <a-divider type="vertical" />
        </template>
        <a-button @click="loadListeners" :loading="loading">
          <template #icon><ReloadOutlined /></template>
          刷新
        </a-button>
        <a-button type="primary" @click="createModalVisible = true">
          <template #icon><PlusOutlined /></template>
          新增监听器
        </a-button>
      </div>
    </div>

    <!-- Table Container -->
    <div class="flex-1 bg-white rounded-lg border border-gray-200 shadow-sm flex flex-col overflow-hidden">
      <a-table
        :columns="columns"
        :data-source="listeners"
        row-key="listener_id"
        :loading="loading"
        :pagination="{ pageSize: 20 }"
        :rowSelection="rowSelection"
        class="w-full flex-1"
        :scroll="{ y: 'max-content' }"
      >
        <!-- Custom Body Cells -->
        <template #bodyCell="{ column, record }">
          <template v-if="column.key === 'kind'">
            <a-tag :color="getKindColor(record.kind)" class="font-medium mr-0">
              {{ formatKind(record.kind) }}
            </a-tag>
          </template>

          <template v-else-if="column.key === 'address'">
            <span class="font-mono text-sm text-slate-600">
              {{ record.bind_host }}:{{ record.bind_port }}
            </span>
          </template>

          <template v-else-if="column.key === 'runtime_status'">
            <div class="flex flex-col gap-0.5">
              <div class="flex items-center gap-2">
                <span class="relative flex h-2.5 w-2.5 shrink-0">
                  <span v-if="record.runtime_status === 'running'" class="absolute inline-flex h-full w-full animate-ping rounded-full bg-green-400 opacity-75"></span>
                  <span class="relative inline-flex h-2.5 w-2.5 rounded-full" 
                        :class="getStatusDotColor(record.runtime_status)"></span>
                </span>
                <span :class="getStatusTextColor(record.runtime_status)" class="capitalize">
                  {{ record.runtime_status }}
                </span>
              </div>
              <a-tooltip v-if="record.last_error" :title="record.last_error">
                <span class="text-xs text-red-400 truncate max-w-[180px] inline-block align-bottom cursor-help">
                  {{ record.last_error }}
                </span>
              </a-tooltip>
            </div>
          </template>

          <template v-else-if="column.key === 'created_at'">
            {{ formatTimestamp(record.created_at) }}
          </template>

          <template v-else-if="column.key === 'action'">
            <div class="flex gap-2 items-center">
              <a-button 
                type="text" size="small" 
                class="text-green-600 hover:text-green-700 hover:bg-green-50"
                @click="doStartListener(record)" 
                v-if="record.enabled === false"
                :loading="actionLoading === record.listener_id + 'start'"
              >
                启动
              </a-button>

              <a-button 
                type="text" size="small" 
                class="text-amber-600 hover:text-amber-700 hover:bg-amber-50"
                @click="doStopListener(record)" 
                v-if="record.enabled === true"
                :loading="actionLoading === record.listener_id + 'stop'"
              >
                停止
              </a-button>

              <a-popconfirm
                title="确定要彻底删除该监听器吗？"
                @confirm="doDeleteListener(record)"
                okText="删除"
                cancelText="取消"
                okType="danger"
              >
                <a-button type="text" size="small" danger :loading="actionLoading === record.listener_id + 'delete'">
                  删除
                </a-button>
              </a-popconfirm>
            </div>
          </template>
        </template>
      </a-table>
    </div>

    <!-- Create Modal -->
    <CreateListenerModal 
      v-model:visible="createModalVisible" 
      @success="loadListeners"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue';
import { message, Modal } from 'ant-design-vue';
import {
  ApiOutlined,
  ReloadOutlined,
  PlusOutlined,
  QuestionCircleOutlined,
} from '@ant-design/icons-vue';
import dayjs from 'dayjs';

import { 
  ListenerRecord, 
  fetchListeners, 
  startListener, 
  stopListener, 
  deleteListener 
} from '@/api/listener';
import { getAuthSettings, updateAuthSettings } from '@/api/settings';
import CreateListenerModal from './components/CreateListenerModal.vue';

const listeners = ref<ListenerRecord[]>([]);
const loading = ref(false);
const createModalVisible = ref(false);
const actionLoading = ref('');

// Batch selection state
const selectedRowKeys = ref<number[]>([]);
const batchStartLoading = ref(false);
const batchStopLoading = ref(false);
const batchDeleteLoading = ref(false);
const rowSelection = computed(() => ({
  selectedRowKeys: selectedRowKeys.value,
  onChange: (keys: number[]) => {
    selectedRowKeys.value = keys;
  },
}));

const columns = [
  { title: '标识 ID', dataIndex: 'listener_id', key: 'listener_id', width: 100 },
  { title: '名称', dataIndex: 'name', key: 'name', width: 200 },
  { title: '协议', dataIndex: 'kind', key: 'kind', width: 100 },
  { title: '侦听地址', key: 'address', width: 200 },
  { title: '当前状态', dataIndex: 'runtime_status', key: 'runtime_status', width: 180 },
  { title: '创建时间', dataIndex: 'created_at', key: 'created_at', width: 180 },
  { title: '操作', key: 'action', width: 140, fixed: 'right' as const },
];

const loadListeners = async () => {
  loading.value = true;
  try {
    const res = await fetchListeners();
    listeners.value = res.listeners || [];
  } catch (err: any) {
    message.error(err.message || '获取监听器列表失败');
  } finally {
    loading.value = false;
  }
};

// Agent Auth Settings
const authForm = reactive({ agent_token: '', agent_auth_mode: 'plain_token' });
const authLoading = ref(false);

const loadAuthSettings = async () => {
  authLoading.value = true;
  try {
    const settings = await getAuthSettings();
    authForm.agent_token = settings.agent_token || '';
    authForm.agent_auth_mode = settings.agent_auth_mode || 'plain_token';
  } catch (_e: any) {
    // silently fail - user can click refresh
  } finally {
    authLoading.value = false;
  }
};

const saveAuthSettings = async () => {
  authLoading.value = true;
  try {
    const result = await updateAuthSettings({
      agent_token: authForm.agent_token,
      agent_auth_mode: authForm.agent_auth_mode
    });
    message.success(result.detail || '认证设置已更新');
  } catch (e: any) {
    message.error(e.message || '保存失败');
  } finally {
    authLoading.value = false;
  }
};

onMounted(() => {
  loadListeners();
  loadAuthSettings();
});

// Batch Actions
async function handleBatchStart() {
  const targets = listeners.value.filter(l => selectedRowKeys.value.includes(l.listener_id) && l.enabled === false);
  if (targets.length === 0) {
    message.info('所选监听器中无需启动的项（已全部启用）');
    return;
  }
  batchStartLoading.value = true;
  try {
    const results = await Promise.allSettled(targets.map(l => startListener(l.listener_id)));
    const succeeded = results.filter(r => r.status === 'fulfilled').length;
    const failed = results.filter(r => r.status === 'rejected').length;
    if (failed === 0) {
      message.success(`成功启动 ${succeeded} 项`);
    } else {
      message.warning(`成功启动 ${succeeded} 项，失败 ${failed} 项`);
    }
    selectedRowKeys.value = [];
    await loadListeners();
  } finally {
    batchStartLoading.value = false;
  }
}

async function handleBatchStop() {
  const targets = listeners.value.filter(l => selectedRowKeys.value.includes(l.listener_id) && l.enabled === true);
  if (targets.length === 0) {
    message.info('所选监听器中无需停止的项（已全部停用）');
    return;
  }
  batchStopLoading.value = true;
  try {
    const results = await Promise.allSettled(targets.map(l => stopListener(l.listener_id)));
    const succeeded = results.filter(r => r.status === 'fulfilled').length;
    const failed = results.filter(r => r.status === 'rejected').length;
    if (failed === 0) {
      message.success(`成功停止 ${succeeded} 项`);
    } else {
      message.warning(`成功停止 ${succeeded} 项，失败 ${failed} 项`);
    }
    selectedRowKeys.value = [];
    await loadListeners();
  } finally {
    batchStopLoading.value = false;
  }
}

function handleBatchDelete() {
  Modal.confirm({
    title: `确认删除选中的 ${selectedRowKeys.value.length} 个监听器？`,
    content: '删除后将不可恢复。',
    okType: 'danger',
    async onOk() {
      batchDeleteLoading.value = true;
      try {
        const results = await Promise.allSettled(
          selectedRowKeys.value.map(id => deleteListener(id))
        );
        const succeeded = results.filter(r => r.status === 'fulfilled').length;
        const failed = results.filter(r => r.status === 'rejected').length;
        if (failed === 0) {
          message.success(`成功删除 ${succeeded} 项`);
        } else {
          message.warning(`成功删除 ${succeeded} 项，失败 ${failed} 项`);
        }
        selectedRowKeys.value = [];
        await loadListeners();
      } finally {
        batchDeleteLoading.value = false;
      }
    }
  });
}

// Row Actions
const doStartListener = async (record: ListenerRecord) => {
  actionLoading.value = record.listener_id + 'start';
  try {
    await startListener(record.listener_id);
    message.success(`监听器 ${record.name} 已启动`);
    await loadListeners();
  } catch (err: any) {
    message.error(err.message || '操作失败');
  } finally {
    actionLoading.value = '';
  }
};

const doStopListener = async (record: ListenerRecord) => {
  actionLoading.value = record.listener_id + 'stop';
  try {
    await stopListener(record.listener_id);
    message.success(`监听器 ${record.name} 已停止`);
    await new Promise(r => setTimeout(r, 2000));
    await loadListeners();
  } catch (err: any) {
    message.error(err.message || '操作失败');
  } finally {
    actionLoading.value = '';
  }
};

const doDeleteListener = async (record: ListenerRecord) => {
  actionLoading.value = record.listener_id + 'delete';
  try {
    await deleteListener(record.listener_id);
    message.success(`监听器 ${record.name} 已删除`);
    await loadListeners();
  } catch (err: any) {
    message.error(err.message || '操作失败');
  } finally {
    actionLoading.value = '';
  }
};

// Formatting Helpers
const formatTimestamp = (ts: number | null | undefined) => {
  if (!ts) return '-';
  // Attempt to detect if it's seconds vs milliseconds
  const ms = ts < 1e12 ? ts * 1000 : ts;
  return dayjs(ms).format('YYYY-MM-DD HH:mm:ss');
};

const getKindColor = (kind: string) => {
  if (kind === 'tcp_json') return 'blue';
  if (kind === 'https_json') return 'purple';
  return 'default';
};

const formatKind = (kind: string) => {
  if (kind === 'tcp_json') return 'TCP';
  if (kind === 'https_json') return 'HTTPS';
  return kind;
};

const getStatusDotColor = (status: string) => {
  if (status === 'running') return 'bg-green-500';
  if (status === 'stopped') return 'bg-slate-400';
  if (status === 'error') return 'bg-red-500';
  if (status === 'starting') return 'bg-blue-400';
  return 'bg-slate-300';
};

const getStatusTextColor = (status: string) => {
  if (status === 'running') return 'text-green-600 font-medium';
  if (status === 'stopped') return 'text-slate-500';
  if (status === 'error') return 'text-red-600 font-medium';
  return 'text-slate-700';
};
</script>

<style scoped>
:deep(.ant-table-wrapper) {
  height: 100%;
}
:deep(.ant-spin-nested-loading) {
  height: 100%;
  display: flex;
  flex-direction: column;
}
:deep(.ant-spin-container) {
  height: 100%;
  display: flex;
  flex-direction: column;
}
:deep(.ant-table) {
  flex-grow: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
}
:deep(.ant-table-container) {
  flex-grow: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
  overflow: hidden;
}
:deep(.ant-table-header) {
  flex-shrink: 0;
}
:deep(.ant-table-body) {
  flex-grow: 1;
  overflow-y: auto !important;
}
</style>
