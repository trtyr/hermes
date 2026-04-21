<template>
  <div class="h-full w-full flex flex-col p-4 relative">
    <!-- Header -->
    <div class="flex justify-between items-center mb-4">
      <h2 class="text-xl font-semibold text-slate-800 dark:text-[var(--text-primary)] flex items-center gap-2 m-0">
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
    <div class="flex-1 bg-white dark:bg-[var(--bg-card)] rounded-lg border border-gray-200 dark:border-[var(--border-default)] shadow-sm flex flex-col overflow-hidden">
      <a-table
        :columns="columns"
        :data-source="listeners"
        row-key="id"
        :loading="loading"
        :pagination="{ pageSize: 20 }"
        class="w-full flex-1"
        :scroll="{ y: 'max-content' }"
      >
        <!-- Custom Body Cells -->
        <template #bodyCell="{ column, record }">
          <template v-if="column.key === 'protocol'">
            <a-tag :color="getProtocolColor(record.protocol)" class="font-medium mr-0">
              {{ record.protocol }}
            </a-tag>
          </template>

          <template v-else-if="column.key === 'address'">
            <span class="font-mono text-sm text-slate-600 dark:text-[var(--text-secondary)]">
              {{ record.bind_host }}:{{ record.bind_port }}
            </span>
          </template>

          <template v-else-if="column.key === 'status'">
            <div class="flex items-center gap-2">
              <span class="relative flex h-2.5 w-2.5 shrink-0">
                <span v-if="record.status === 'running'" class="absolute inline-flex h-full w-full animate-ping rounded-full bg-green-400 opacity-75"></span>
                <span class="relative inline-flex h-2.5 w-2.5 rounded-full" 
                      :class="getStatusDotColor(record.status)"></span>
              </span>
              <span :class="getStatusTextColor(record.status)" class="capitalize">
                {{ record.status }}
              </span>
            </div>
          </template>

          <template v-else-if="column.key === 'created_at'">
            {{ formatTimestamp(record.created_at) }}
          </template>

          <template v-else-if="column.key === 'action'">
            <div class="flex gap-2 items-center">
              <a-button 
                type="text" size="small" 
                class="text-green-600 dark:text-green-400 hover:text-green-700 hover:bg-green-50 dark:hover:bg-green-900/30"
                @click="doStartListener(record)" 
                v-if="record.status !== 'running'"
                :loading="actionLoading === record.id + 'start'"
              >
                启动
              </a-button>

              <a-button 
                type="text" size="small" 
                class="text-amber-600 dark:text-amber-500 hover:text-amber-700 hover:bg-amber-50 dark:hover:bg-amber-900/30"
                @click="doStopListener(record)" 
                v-if="record.status === 'running'"
                :loading="actionLoading === record.id + 'stop'"
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
                <a-button type="text" size="small" danger :loading="actionLoading === record.id + 'delete'">
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
  { title: '标识 ID', dataIndex: 'id', key: 'id', width: 120, ellipsis: true },
  { title: '名称', dataIndex: 'name', key: 'name', width: 200 },
  { title: '协议', dataIndex: 'protocol', key: 'protocol', width: 100 },
  { title: '侦听地址', key: 'address', width: 200 },
  { title: '当前状态', dataIndex: 'status', key: 'status', width: 140 },
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
  actionLoading.value = record.id + 'start';
  try {
    await startListener(record.id);
    message.success(`监听器 ${record.name} 已启动`);
    await loadListeners();
  } catch (err: any) {
    message.error(err.message || '操作失败');
  } finally {
    actionLoading.value = '';
  }
};

const doStopListener = async (record: ListenerRecord) => {
  actionLoading.value = record.id + 'stop';
  try {
    await stopListener(record.id);
    message.success(`监听器 ${record.name} 已停止`);
    await loadListeners();
  } catch (err: any) {
    message.error(err.message || '操作失败');
  } finally {
    actionLoading.value = '';
  }
};

const doDeleteListener = async (record: ListenerRecord) => {
  actionLoading.value = record.id + 'delete';
  try {
    await deleteListener(record.id);
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

const getProtocolColor = (proto: string) => {
  const p = proto?.toUpperCase() || '';
  if (p === 'TCP') return 'blue';
  if (p === 'HTTP' || p === 'HTTPS') return 'purple';
  if (p === 'DNS') return 'cyan';
  return 'default';
};

const getStatusDotColor = (status: string) => {
  if (status === 'running') return 'bg-green-500';
  if (status === 'stopped') return 'bg-slate-400';
  if (status === 'error') return 'bg-red-500';
  if (status === 'starting') return 'bg-blue-400';
  return 'bg-slate-300';
};

const getStatusTextColor = (status: string) => {
  if (status === 'running') return 'text-green-600 dark:text-green-500 font-medium';
  if (status === 'stopped') return 'text-slate-500 dark:text-[var(--text-secondary)]';
  if (status === 'error') return 'text-red-600 dark:text-red-500 font-medium';
  return 'text-slate-700 dark:text-[var(--text-secondary)]';
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
