<template>
  <div class="h-full w-full flex flex-col p-4 relative">
    <!-- Header -->
    <div class="flex justify-between items-center mb-4">
      <h2 class="text-xl font-semibold text-slate-800 dark:text-slate-100 flex items-center gap-2 m-0">
        <FileSearchOutlined class="text-cyan-500" />
        操作日志
      </h2>
      <div class="flex gap-2">
        <a-button @click="loadAudits" :loading="loading">
          <template #icon><ReloadOutlined /></template>
          刷新
        </a-button>
      </div>
    </div>

    <!-- Filter Bar -->
    <div class="mb-4 flex flex-wrap gap-3 items-center">
      <a-input
        v-model:value="filterForm.operator"
        placeholder="操作者"
        allowClear
        class="!w-40"
        @pressEnter="applyFilter"
      />
      <a-select
        v-model:value="filterForm.action"
        allowClear
        placeholder="操作类型"
        class="!w-48"
        @change="applyFilter"
      >
        <a-select-option v-for="a in actionTypes" :key="a" :value="a">
          {{ formatAction(a) }}
        </a-select-option>
      </a-select>
      <a-select
        v-model:value="filterForm.target_kind"
        allowClear
        placeholder="目标类型"
        class="!w-36"
        @change="applyFilter"
      >
        <a-select-option v-for="k in targetKinds" :key="k" :value="k">
          {{ formatTargetKind(k) }}
        </a-select-option>
      </a-select>
      <a-input
        v-model:value="filterForm.target_id"
        placeholder="目标 ID"
        allowClear
        class="!w-32"
        @pressEnter="applyFilter"
      />
      <a-button type="primary" @click="applyFilter">
        <template #icon><SearchOutlined /></template>
        查询
      </a-button>
      <a-button @click="resetFilter">重置</a-button>
    </div>

    <!-- Table Container -->
    <div class="flex-1 bg-white dark:bg-[#1C1E22] rounded-lg border border-gray-200 dark:border-[#14161A] shadow-sm flex flex-col overflow-hidden">
      <a-table
        :columns="columns"
        :data-source="audits"
        row-key="audit_id"
        :loading="loading"
        :pagination="{
          pageSize: pageSize,
          total: total,
          current: currentPage,
          onChange: onPageChange,
          showTotal: (t: number) => `共 ${t} 条`,
          showSizeChanger: false,
        }"
        class="w-full flex-1"
        :scroll="{ y: 'max-content' }"
      >
        <!-- Custom Body Cells -->
        <template #bodyCell="{ column, record }">
          <template v-if="column.key === 'audit_id'">
            <span class="font-mono text-sm text-slate-400">#{{ record.audit_id }}</span>
          </template>

          <template v-else-if="column.key === 'operator'">
            <span class="font-medium text-slate-700 dark:text-slate-300">
              {{ record.operator }}
            </span>
          </template>

          <template v-else-if="column.key === 'action'">
            <a-tag :color="getActionColor(record.action)" class="font-medium mr-0">
              {{ formatAction(record.action) }}
            </a-tag>
          </template>

          <template v-else-if="column.key === 'target'">
            <div class="flex items-center gap-2">
              <a-tag size="small" class="mr-0">
                {{ formatTargetKind(record.target_kind) }}
              </a-tag>
              <span v-if="record.target_id" class="font-mono text-sm text-slate-500 dark:text-slate-400">
                {{ record.target_id }}
              </span>
            </div>
          </template>

          <template v-else-if="column.key === 'detail'">
            <a-tooltip v-if="record.detail" :title="record.detail">
              <span class="text-sm text-slate-500 dark:text-slate-400 truncate max-w-[260px] inline-block align-bottom">
                {{ record.detail }}
              </span>
            </a-tooltip>
            <span v-else class="text-slate-400">-</span>
          </template>

          <template v-else-if="column.key === 'created_at'">
            {{ formatTimestamp(record.created_at) }}
          </template>
        </template>
      </a-table>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { message } from 'ant-design-vue';
import {
  FileSearchOutlined,
  ReloadOutlined,
  SearchOutlined,
} from '@ant-design/icons-vue';
import dayjs from 'dayjs';

import { AuditRecord, fetchAudits } from '@/api/audit';

const audits = ref<AuditRecord[]>([]);
const loading = ref(false);
const total = ref(0);
const currentPage = ref(1);
const pageSize = 25;

// Filter
const filterForm = ref({
  operator: undefined as string | undefined,
  action: undefined as string | undefined,
  target_kind: undefined as string | undefined,
  target_id: undefined as string | undefined,
});

// All known action types from server
const actionTypes = [
  'dispatch_task',
  'broadcast_task',
  'cancel_task',
  'open_command_session',
  'queue_command_session',
  'execute_command_session',
  'close_command_session',
  'disconnect_agent',
  'disable_agent',
  'enable_agent',
  'delete_agent',
  'upload_file',
  'download_file',
  'create_listener',
  'update_listener',
  'enable_listener',
  'disable_listener',
  'delete_listener',
  'create_listener_agent_build',
  'create_agent_build',
  'update_beacon_config',
  'open_terminal_session',
  'queue_terminal_command',
  'close_terminal_session',
];

const targetKinds = [
  'agent',
  'task',
  'command_session',
  'listener',
  'agent_build',
  'terminal_session',
];

const columns = [
  { title: 'ID', dataIndex: 'audit_id', key: 'audit_id', width: 80 },
  { title: '操作者', dataIndex: 'operator', key: 'operator', width: 120 },
  { title: '操作', dataIndex: 'action', key: 'action', width: 170 },
  { title: '目标', key: 'target', width: 200 },
  { title: '详情', dataIndex: 'detail', key: 'detail', width: 280, ellipsis: true },
  { title: '时间', dataIndex: 'created_at', key: 'created_at', width: 170 },
];

const loadAudits = async () => {
  loading.value = true;
  try {
    const filter: Record<string, any> = {
      limit: pageSize,
      offset: (currentPage.value - 1) * pageSize,
    };
    if (filterForm.value.operator) filter.operator = filterForm.value.operator;
    if (filterForm.value.action) filter.action = filterForm.value.action;
    if (filterForm.value.target_kind) filter.target_kind = filterForm.value.target_kind;
    if (filterForm.value.target_id) filter.target_id = filterForm.value.target_id;

    const res = await fetchAudits(filter);
    audits.value = res.audits || [];
    total.value = res.total || 0;
  } catch (err: any) {
    message.error(err.message || '获取审计日志失败');
  } finally {
    loading.value = false;
  }
};

const onPageChange = (page: number) => {
  currentPage.value = page;
  loadAudits();
};

const applyFilter = () => {
  currentPage.value = 1;
  loadAudits();
};

const resetFilter = () => {
  filterForm.value = {
    operator: undefined,
    action: undefined,
    target_kind: undefined,
    target_id: undefined,
  };
  currentPage.value = 1;
  loadAudits();
};

onMounted(() => {
  loadAudits();
});

// Formatting Helpers
const formatTimestamp = (ts: number | null | undefined) => {
  if (!ts) return '-';
  const ms = ts < 1e12 ? ts * 1000 : ts;
  return dayjs(ms).format('YYYY-MM-DD HH:mm:ss');
};

const formatAction = (action: string) => {
  return action?.replace(/_/g, ' ').replace(/\b\w/g, c => c.toUpperCase()) || '-';
};

const formatTargetKind = (kind: string) => {
  const map: Record<string, string> = {
    agent: 'Agent',
    task: 'Task',
    command_session: 'Session',
    listener: 'Listener',
    agent_build: 'Build',
    terminal_session: 'Terminal',
  };
  return map[kind] || kind || '-';
};

const getActionColor = (action: string) => {
  if (action?.includes('create') || action?.includes('enable') || action?.includes('open')) return 'green';
  if (action?.includes('delete') || action?.includes('disable') || action?.includes('disconnect') || action?.includes('close')) return 'red';
  if (action?.includes('update') || action?.includes('dispatch') || action?.includes('queue') || action?.includes('execute')) return 'blue';
  if (action?.includes('cancel')) return 'orange';
  if (action?.includes('upload') || action?.includes('download')) return 'purple';
  return 'default';
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
