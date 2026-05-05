<template>
  <div class="h-full w-full flex flex-col p-4 relative">
    <!-- Header Actions -->
    <div class="flex justify-between items-center mb-4">
      <h2 class="text-xl font-semibold text-slate-800">Agent管理</h2>
      <div class="flex items-center gap-2">
        <template v-if="selectedRowKeys.length > 0">
          <span class="text-sm text-slate-500">已选中 {{ selectedRowKeys.length }} 项</span>
          <a-button type="link" size="small" @click="selectedRowKeys = []">取消选择</a-button>
          <a-button :loading="batchLoading" @click="handleBatchDisable">
            <template #icon><StopOutlined /></template>
            批量禁用
          </a-button>
          <a-button :loading="batchLoading" @click="handleBatchEnable">
            <template #icon><CheckCircleOutlined /></template>
            批量启用
          </a-button>
          <a-button danger :loading="batchLoading" @click="handleBatchDelete">
            <template #icon><DeleteOutlined /></template>
            批量删除
          </a-button>
          <a-divider type="vertical" />
        </template>
        <a-input-search
          v-model:value="searchKeyword"
          placeholder="搜索节点..."
          style="width: 250px"
          @search="onSearch"
          allowClear
        />
        <a-button @click="loadAgents">
          <template #icon><ReloadOutlined /></template>
          刷新
        </a-button>
      </div>
    </div>

    <!-- Table -->
    <div class="flex-1 bg-white rounded-lg border border-gray-200 shadow-sm overflow-hidden flex flex-col">
      <a-table
        :dataSource="agents"
        :columns="columns"
        :loading="loading"
        :pagination="pagination"
        :customRow="customRow"
        :rowSelection="rowSelection"
        @change="handleTableChange"
        size="middle"
        :scroll="{ x: 'max-content', y: 'calc(100vh - 280px)' }"
        rowKey="agent_id"
        class="w-full h-full"
      >
        <template #bodyCell="{ column, record }">
          <template v-if="column.key === 'hostname'">
            <a-button type="link" class="p-0 font-medium" @click="openSession(record)">
              {{ record.hostname || record.agent_id }}
            </a-button>
          </template>

          <template v-else-if="column.key === 'status'">
            <div class="flex flex-col gap-1">
              <a-badge :status="record.is_online ? 'success' : 'default'" :text="record.is_online ? '在线' : '离线'" />
              <a-badge v-if="record.is_disabled" status="error" text="已禁用" />
            </div>
          </template>

          <template v-else-if="column.key === 'user'">
            <span class="text-sm">{{ record.username || '-' }}</span>
          </template>

          <template v-else-if="column.key === 'platform'">
            <div class="flex items-center gap-1.5">
              <WindowsOutlined class="text-blue-500" />
              <span>{{ record.os }} {{ record.arch }}</span>
            </div>
          </template>

          <template v-else-if="column.key === 'network'">
            <div class="text-xs space-y-0.5">
              <div v-if="record.internal_ip"><span class="text-slate-400">内网:</span> {{ record.internal_ip }}</div>
              <div v-if="record.external_ip"><span class="text-slate-400">外网:</span> {{ record.external_ip }}</div>
            </div>
          </template>

          <template v-else-if="column.key === 'beacon'">
            <div class="text-xs">{{ record.sleep_interval }}s ±{{ record.jitter }}%</div>
          </template>

          <template v-else-if="column.key === 'privilege'">
            <a-tooltip v-if="record.privilege" :title="record.privilege">
              <SafetyCertificateOutlined
                :style="{ color: record.privilege.startsWith('Admin') || record.privilege === 'SYSTEM' ? '#f5222d' : '#8c8c8c', fontSize: '16px', cursor: 'pointer' }"
              />
            </a-tooltip>
            <span v-else class="text-slate-400">-</span>
          </template>

          <template v-else-if="column.key === 'last_seen'">
            <span class="text-xs">{{ formatTimestamp(record.last_seen) }}</span>
          </template>
        </template>
      </a-table>
    </div>

    <!-- Context menu (simplified) -->
    <AgentContextMenu
      :visible="contextMenuState.visible"
      :x="contextMenuState.x"
      :y="contextMenuState.y"
      :agent="contextMenuState.record"
      @close="contextMenuState.visible = false"
      @action="handleAction"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, onBeforeUnmount } from 'vue';
import { useRouter } from 'vue-router';
import { message, Modal } from 'ant-design-vue';
import { ReloadOutlined, WindowsOutlined, SafetyCertificateOutlined, DeleteOutlined, StopOutlined, CheckCircleOutlined } from '@ant-design/icons-vue';
import { fetchAgents, disconnectAgent, deleteAgent, disableAgent, enableAgent } from '@/api/agent';
import type { Agent } from '@/api/agent';
import { formatTimestamp } from '@/utils/format';
import { useAgentWebSocket } from './hooks/useAgentWebSocket';
import AgentContextMenu from './components/AgentContextMenu.vue';

const router = useRouter();

// Core State
const agents = ref<Agent[]>([]);
const loading = ref(false);

// Batch selection state
const selectedRowKeys = ref<string[]>([]);
const batchLoading = ref(false);
const rowSelection = computed(() => ({
  selectedRowKeys: selectedRowKeys.value,
  onChange: (keys: string[]) => {
    selectedRowKeys.value = keys;
  },
}));
const searchKeyword = ref('');
const pagination = reactive({ current: 1, pageSize: 20, total: 0, showSizeChanger: true });

// Context menu state
const contextMenuState = reactive({ visible: false, x: 0, y: 0, record: null as Agent | null });

const columns = [
  { title: '主机名', dataIndex: 'hostname', key: 'hostname', width: 130 },
  { title: '状态', key: 'status', width: 80 },
  { title: '用户', key: 'user', width: 100 },
  { title: '平台', key: 'platform', width: 130 },
  { title: '网络地址', key: 'network', width: 200 },
  { title: '监听器', dataIndex: 'listener_name', key: 'listener_name', width: 100 },
  { title: 'Beacon', key: 'beacon', width: 90 },
  { title: 'PID', dataIndex: 'pid', key: 'pid', width: 60 },
  { title: '权限', key: 'privilege', width: 70 },
  { title: '最后活跃', key: 'last_seen', width: 150 },
];

// WebSocket hook (keeps agent list fresh)
const selectedAgent = ref<Agent | null>(null);
const detailVisible = ref(false);
useAgentWebSocket(agents, selectedAgent, detailVisible, loadAgents);

async function handleBatchDelete() {
  Modal.confirm({
    title: `确认删除选中的 ${selectedRowKeys.value.length} 个节点？`,
    content: '删除后将不可恢复。在线节点需先禁用后才能删除。',
    okType: 'danger',
    async onOk() {
      batchLoading.value = true;
      try {
        const results = await Promise.allSettled(
          selectedRowKeys.value.map(id => deleteAgent(id))
        );
        const succeeded = results.filter(r => r.status === 'fulfilled').length;
        const failed = results.filter(r => r.status === 'rejected').length;
        if (failed === 0) {
          message.success(`成功删除 ${succeeded} 项`);
        } else {
          message.warning(`成功 ${succeeded} 项，失败 ${failed} 项（在线节点请先禁用）`);
        }
        selectedRowKeys.value = [];
        loadAgents();
      } finally {
        batchLoading.value = false;
      }
    }
  });
}

async function handleBatchDisable() {
  Modal.confirm({
    title: `确认禁用选中的 ${selectedRowKeys.value.length} 个节点？`,
    content: '禁用后节点将无法连接。',
    async onOk() {
      batchLoading.value = true;
      try {
        const results = await Promise.allSettled(
          selectedRowKeys.value.map(id => disableAgent(id))
        );
        const succeeded = results.filter(r => r.status === 'fulfilled').length;
        const failed = results.filter(r => r.status === 'rejected').length;
        if (failed === 0) {
          message.success(`成功禁用 ${succeeded} 项`);
        } else {
          message.warning(`成功 ${succeeded} 项，失败 ${failed} 项`);
        }
        selectedRowKeys.value = [];
        loadAgents();
      } finally {
        batchLoading.value = false;
      }
    }
  });
}

async function handleBatchEnable() {
  Modal.confirm({
    title: `确认启用选中的 ${selectedRowKeys.value.length} 个节点？`,
    content: '启用后节点可以重新连接。',
    async onOk() {
      batchLoading.value = true;
      try {
        const results = await Promise.allSettled(
          selectedRowKeys.value.map(id => enableAgent(id))
        );
        const succeeded = results.filter(r => r.status === 'fulfilled').length;
        const failed = results.filter(r => r.status === 'rejected').length;
        if (failed === 0) {
          message.success(`成功启用 ${succeeded} 项`);
        } else {
          message.warning(`成功 ${succeeded} 项，失败 ${failed} 项`);
        }
        selectedRowKeys.value = [];
        loadAgents();
      } finally {
        batchLoading.value = false;
      }
    }
  });
}

async function loadAgents() {
  loading.value = true;
  try {
    const res = await fetchAgents({
      limit: pagination.pageSize,
      offset: (pagination.current - 1) * pagination.pageSize,
      keyword: searchKeyword.value || undefined
    });
    agents.value = res.agents || [];
    pagination.total = res.total || 0;
  } catch (error: any) {
    message.error(error.message || '获取节点列表失败');
  } finally {
    loading.value = false;
  }
}

function handleTableChange(pag: any) {
  pagination.current = pag.current;
  pagination.pageSize = pag.pageSize;
  loadAgents();
}

function onSearch() {
  pagination.current = 1;
  loadAgents();
}

function openSession(agent: Agent) {
  router.push(`/agent/${agent.agent_id}/session`);
}

function handleAction({ action, agent }: { action: string, agent: Agent }) {
  const actionMap: Record<string, { title: string, func: Function }> = {
    'disconnect': { title: '断开连接', func: disconnectAgent },
    'disable': { title: '禁用', func: disableAgent },
    'enable': { title: '启用', func: enableAgent },
    'delete': { title: '删除记录', func: deleteAgent },
  };

  const target = actionMap[action];
  if (!target) return;

  Modal.confirm({
    title: `确认要对节点 ${agent.agent_id} 执行 [${target.title}] 操作吗？`,
    content: action === 'delete' ? '删除后将不可恢复。' : action === 'disable' ? '禁用后节点将无法连接。' : '',
    okType: action === 'delete' || action === 'disconnect' || action === 'disable' ? 'danger' : 'primary',
    async onOk() {
      try {
        await target.func(agent.agent_id);
        message.success(`操作 [${target.title}] 执行成功`);
        loadAgents();
      } catch (e: any) {
        message.error(e.message);
      }
    }
  });
}

const customRow = (record: Agent) => {
  return {
    onContextmenu: (e: MouseEvent) => {
      e.preventDefault();
      contextMenuState.x = e.clientX;
      contextMenuState.y = e.clientY;
      contextMenuState.record = record;
      contextMenuState.visible = true;
    }
  };
};

function closeContextMenu() {
  contextMenuState.visible = false;
}

onMounted(() => {
  loadAgents();
  document.addEventListener('click', closeContextMenu);
});

onBeforeUnmount(() => {
  document.removeEventListener('click', closeContextMenu);
});
</script>
