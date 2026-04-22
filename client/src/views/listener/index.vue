<template>
  <div class="h-full w-full flex flex-col p-4 relative">
    <!-- Header -->
    <div class="flex justify-between items-center mb-4">
      <h2 class="text-xl font-semibold text-slate-800 flex items-center gap-2 m-0">
        <ApiOutlined class="text-indigo-500" />
        监听器管理
      </h2>
      <div class="flex gap-2">
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
import { ref, onMounted } from 'vue';
import { message } from 'ant-design-vue';
import { 
  ApiOutlined, 
  ReloadOutlined, 
  PlusOutlined,
} from '@ant-design/icons-vue';
import dayjs from 'dayjs';

import { 
  ListenerRecord, 
  fetchListeners, 
  startListener, 
  stopListener, 
  deleteListener 
} from '@/api/listener';
import CreateListenerModal from './components/CreateListenerModal.vue';

const listeners = ref<ListenerRecord[]>([]);
const loading = ref(false);
const createModalVisible = ref(false);
const actionLoading = ref('');

const columns = [
  { title: '标识 ID', dataIndex: 'listener_id', key: 'listener_id', width: 100 },
  { title: '名称', dataIndex: 'name', key: 'name', width: 200 },
  { title: '协议', dataIndex: 'kind', key: 'kind', width: 100 },
  { title: '侦听地址', key: 'address', width: 200 },
  { title: '当前状态', dataIndex: 'runtime_status', key: 'runtime_status', width: 180 },
  { title: '创建时间', dataIndex: 'created_at', key: 'created_at', width: 180 },
  { title: '操作', key: 'action', width: 220, fixed: 'right' as const },
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

onMounted(() => {
  loadListeners();
});

// Row Actions
const doStartListener = async (record: ListenerRecord) => {
  actionLoading.value = record.listener_id + 'start';
  try {
    await startListener(record.listener_id);
    message.success(`监听器 ${record.name} 已启动`);
    await new Promise(r => setTimeout(r, 2000));
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
}
:deep(.ant-table-body) {
  overflow-y: auto !important;
}
</style>
