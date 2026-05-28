<template>
  <div class="h-full w-full flex flex-col p-4 relative">
    <!-- Header -->
    <div class="flex justify-between items-center mb-4">
      <h2 class="text-xl font-semibold text-slate-800 flex items-center gap-2 m-0">
        <RocketOutlined class="text-orange-500" />
        载荷生成
      </h2>
      <div class="flex gap-2">
        <template v-if="selectedRowKeys.length > 0">
          <span class="text-sm text-slate-500">已选中 {{ selectedRowKeys.length }} 项</span>
          <a-button type="link" size="small" @click="selectedRowKeys = []">取消选择</a-button>
          <a-button danger :loading="batchDeleteLoading" @click="handleBatchDelete">
            <template #icon><DeleteOutlined /></template>
            批量删除
          </a-button>
          <a-divider type="vertical" />
        </template>
        <a-button @click="loadBuilds" :loading="loading">
          <template #icon><ReloadOutlined /></template>
          刷新
        </a-button>
        <a-button type="primary" @click="buildModalVisible = true">
          <template #icon><PlusOutlined /></template>
          新建构建
        </a-button>
      </div>
    </div>

    <!-- Table Container -->
    <div class="flex-1 bg-white rounded-lg border border-gray-200 shadow-sm flex flex-col overflow-hidden">
      <a-table
        :columns="columns"
        :data-source="builds"
        row-key="build_id"
        :loading="loading"
        :pagination="{ pageSize: 20, total: total, current: currentPage, onChange: onPageChange }"
        :rowSelection="rowSelection"
        class="w-full flex-1"
        :scroll="{ x: 1020 }"
      >
        <!-- Custom Body Cells -->
        <template #bodyCell="{ column, record }">
          <template v-if="column.key === 'build_id'">
            <span class="font-mono text-sm">#{{ record.build_id }}</span>
          </template>

          <template v-else-if="column.key === 'target_triple'">
            <span class="font-mono text-sm text-slate-600">
              {{ record.target_triple }}
            </span>
          </template>

          <template v-else-if="column.key === 'profile'">
            <a-tag :color="record.profile === 'release' ? 'green' : 'blue'" class="font-medium mr-0">
              {{ record.profile }}
            </a-tag>
          </template>

          <template v-else-if="column.key === 'status'">
            <div class="flex items-center gap-2">
              <span class="relative flex h-2.5 w-2.5 shrink-0">
                <span v-if="record.status === 'pending'" class="absolute inline-flex h-full w-full animate-ping rounded-full bg-blue-400 opacity-75"></span>
                <span class="relative inline-flex h-2.5 w-2.5 rounded-full"
                      :class="getStatusDotColor(record.status)"></span>
              </span>
              <span :class="getStatusTextColor(record.status)">
                {{ getStatusLabel(record.status) }}
              </span>
            </div>
          </template>

          <template v-else-if="column.key === 'listener_id'">
            <span v-if="record.listener_id" class="text-sm">
              {{ listeners.find(l => l.listener_id === record.listener_id)?.name || `#${record.listener_id}` }}
            </span>
            <span v-else class="text-slate-400">-</span>
          </template>

          <template v-else-if="column.key === 'server_addr'">
            <span class="font-mono text-sm text-slate-600">
              {{ record.server_addr || '-' }}
            </span>
          </template>

          <template v-else-if="column.key === 'created_at'">
            {{ formatTimestamp(record.created_at) }}
          </template>

          <template v-else-if="column.key === 'detail'">
            <a-tooltip v-if="record.detail" :title="record.detail">
              <span class="text-sm text-slate-500 truncate max-w-[200px] inline-block align-bottom">
                {{ record.detail }}
              </span>
            </a-tooltip>
            <span v-else class="text-slate-400">-</span>
          </template>

          <template v-else-if="column.key === 'action'">
            <div class="flex items-center gap-2">
              <a-button
                type="link"
                size="small"
                @click="openBuildLog(record)"
              >
                <template #icon><FileTextOutlined /></template>
                查看日志
              </a-button>
              <a-button
                v-if="record.status === 'succeeded'"
                type="link"
                size="small"
                @click="handleDownload(record)"
              >
                <template #icon><DownloadOutlined /></template>
                下载
              </a-button>
              <a-popconfirm
                v-if="record.status !== 'pending'"
                title="确定删除此构建？"
                ok-text="删除"
                cancel-text="取消"
                @confirm="handleDelete(record)"
              >
                <a-button type="link" size="small" danger>
                  <template #icon><DeleteOutlined /></template>
                  删除
                </a-button>
              </a-popconfirm>
            </div>
          </template>
        </template>
      </a-table>
    </div>

    <!-- Build Log Drawer -->
    <BuildLogDrawer
      v-model:visible="logDrawerVisible"
      :build-id="logBuildId"
      :initial-status="logBuildStatus"
      :initial-detail="logBuildDetail"
      @completed="loadBuilds"
    />

    <!-- Build Modal -->
    <a-modal
      v-model:open="buildModalVisible"
      title="新建载荷构建"
      width="600px"
      :confirm-loading="building"
      @ok="handleBuild"
      ok-text="开始构建"
      cancel-text="取消"
      :destroyOnClose="true"
    >
      <a-form layout="vertical" class="mt-4">
        <a-form-item label="目标平台" required>
          <a-select
            v-model:value="buildForm.target_triple"
            allowClear
            placeholder="选择目标平台"
          >
            <a-select-option value="x86_64-pc-windows-msvc">Windows x86_64</a-select-option>
            <a-select-option value="i686-pc-windows-msvc">Windows x86</a-select-option>
          </a-select>
        </a-form-item>

        <a-form-item label="绑定监听器" help="推荐选择，用于确定通信协议和回连地址">
          <a-select
            v-model:value="buildForm.listener_id"
            allowClear
            placeholder="选择监听器"
            :loading="listenersLoading"
          >
            <a-select-option v-for="l in listeners" :key="l.listener_id" :value="l.listener_id">
              <span class="font-mono mr-2">#{{ l.listener_id }}</span>
              {{ l.name }}
              <a-tag :color="getKindColor(l.kind)" size="small" class="ml-2 mr-0">
                {{ formatKind(l.kind) }}
              </a-tag>
            </a-select-option>
          </a-select>
        </a-form-item>

        <a-form-item label="回连地址" help="留空则使用监听器绑定地址">
          <a-input
            v-model:value="buildForm.server_addr"
            placeholder="例: 192.168.1.100:4444"
            class="font-mono"
          />
        </a-form-item>

        <a-form-item label="Agent Token" help="留空则不嵌入默认认证令牌">
          <a-input-password
            v-model:value="buildForm.agent_token"
            placeholder="可选，嵌入编译时的认证令牌"
            class="font-mono"
          />
        </a-form-item>

        <a-form-item label="心跳间隔" help="Agent 回连间隔（秒），默认 15 秒">
          <a-input-number
            v-model:value="buildForm.heartbeat_secs"
            :min="1"
            :max="86400"
            placeholder="15"
            class="w-full"
          />
        </a-form-item>

        <a-form-item label="抖动系数" help="心跳随机偏移百分比（0-100），默认 0">
          <a-input-number
            v-model:value="buildForm.jitter"
            :min="0"
            :max="100"
            placeholder="0"
            class="w-full"
          />
        </a-form-item>
      </a-form>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { message, Modal } from 'ant-design-vue';
import {
  RocketOutlined,
  ReloadOutlined,
  PlusOutlined,
  DownloadOutlined,
  DeleteOutlined,
  FileTextOutlined,
} from '@ant-design/icons-vue';
import dayjs from 'dayjs';

import {
  AgentBuildRecord,
  fetchAgentBuilds,
  createAgentBuild,
  deleteAgentBuild,
} from '@/api/agentBuild';
import { fetchListeners, ListenerRecord } from '@/api/listener';
import { useConnectionStore } from '@/store/connection';
import { useEventStore } from '@/store/events';
import { apiFetchBlob } from '@/api/request';
import BuildLogDrawer from './components/BuildLogDrawer.vue';

const builds = ref<AgentBuildRecord[]>([]);
const loading = ref(false);
const total = ref(0);
const currentPage = ref(1);
const pageSize = 20;

// Batch selection state
const selectedRowKeys = ref<number[]>([]);
const batchDeleteLoading = ref(false);
const rowSelection = computed(() => ({
  selectedRowKeys: selectedRowKeys.value,
  onChange: (keys: number[]) => {
    selectedRowKeys.value = keys;
  },
  getCheckboxProps: (record: AgentBuildRecord) => ({
    disabled: record.status === 'pending',
  }),
}));

// Build form
const buildModalVisible = ref(false);
const building = ref(false);
const buildForm = ref({
  target_triple: undefined as string | undefined,
  listener_id: undefined as number | undefined,
  server_addr: '',
  agent_token: '',
  profile: 'release',
  heartbeat_secs: undefined as number | undefined,
  jitter: undefined as number | undefined,
});

// Listeners for the form dropdown
const listeners = ref<ListenerRecord[]>([]);
const listenersLoading = ref(false);
const eventStore = useEventStore();

// Log drawer state
const logDrawerVisible = ref(false);
const logBuildId = ref<number | null>(null);
const logBuildStatus = ref('');
const logBuildDetail = ref('');

const columns = [
  { title: 'ID', dataIndex: 'build_id', key: 'build_id', width: 50 },
  { title: '目标平台', dataIndex: 'target_triple', key: 'target_triple', width: 160 },
  { title: '监听器', dataIndex: 'listener_id', key: 'listener_id', width: 100 },
  { title: '回连地址', dataIndex: 'server_addr', key: 'server_addr', width: 150 },
  { title: '状态', dataIndex: 'status', key: 'status', width: 70 },
  { title: '创建时间', dataIndex: 'created_at', key: 'created_at', width: 150 },
  { title: '操作', key: 'action', width: 120, fixed: 'right' },
];

async function handleBatchDelete() {
  Modal.confirm({
    title: `确认删除选中的 ${selectedRowKeys.value.length} 个构建？`,
    content: '删除后将不可恢复。',
    okType: 'danger',
    async onOk() {
      batchDeleteLoading.value = true;
      try {
        const results = await Promise.allSettled(
          selectedRowKeys.value.map(id => deleteAgentBuild(id))
        );
        const succeeded = results.filter(r => r.status === 'fulfilled').length;
        const failed = results.filter(r => r.status === 'rejected').length;
        if (failed === 0) {
          message.success(`成功删除 ${succeeded} 项`);
        } else {
          message.warning(`成功删除 ${succeeded} 项，失败 ${failed} 项`);
        }
        selectedRowKeys.value = [];
        loadBuilds();
      } finally {
        batchDeleteLoading.value = false;
      }
    }
  });
}

const loadBuilds = async () => {
  loading.value = true;
  try {
    const res = await fetchAgentBuilds({
      limit: pageSize,
      offset: (currentPage.value - 1) * pageSize,
    });
    builds.value = (res.builds || []).filter((b: AgentBuildRecord) => b && b.build_id);
    total.value = res.total || 0;
  } catch (err: any) {
    message.error(err.message || '获取构建列表失败');
  } finally {
    loading.value = false;
  }
};

async function handleDownload(record: AgentBuildRecord) {
  try {
    const blob = await apiFetchBlob(`/agent-builds/${record.build_id}/download`);
    const downloadUrl = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = downloadUrl;
    link.download = record.artifact_name || `agent-build-${record.build_id}`;
    link.click();
    URL.revokeObjectURL(downloadUrl);
  } catch (e: any) {
    message.error(e.message || '下载失败');
  }
}

async function handleDelete(record: AgentBuildRecord) {
  try {
    await deleteAgentBuild(record.build_id);
    message.success(`构建 #${record.build_id} 已删除`);
    loadBuilds();
  } catch (e: any) {
    message.error(e.message || '删除失败');
  }
}

const loadListeners = async () => {
  listenersLoading.value = true;
  try {
    const res = await fetchListeners();
    listeners.value = res.listeners || [];
  } catch {
    // Silently fail — listeners are optional for the form
  } finally {
    listenersLoading.value = false;
  }
};

const onPageChange = (page: number) => {
  currentPage.value = page;
  loadBuilds();
};

let unsubscribe: (() => void) | null = null;

onMounted(() => {
  loadBuilds();
  loadListeners();
  
  unsubscribe = eventStore.subscribe((event) => {
    if (event.type === 'agent_build_created' || event.type === 'agent_build_completed' || event.type === 'agent_build_deleted') {
      loadBuilds();
      
      if (event.type === 'agent_build_completed') {
        if (event.status === 'succeeded') {
          message.success(`构建 #${event.build_id} 完成！`);
        } else if (event.status === 'failed') {
          message.error(`构建 #${event.build_id} 失败`);
        }
      }
    }
  });
});

onUnmounted(() => {
  if (unsubscribe) unsubscribe();
});

// Build action
const handleBuild = async () => {
  building.value = true;
  try {
    if (!buildForm.value.target_triple) {
      message.warning('请选择目标平台');
      building.value = false;
      return;
    }
    const data: Record<string, any> = {
      profile: buildForm.value.profile,
    };
    if (buildForm.value.target_triple) data.target_triple = buildForm.value.target_triple;
    if (buildForm.value.listener_id) data.listener_id = buildForm.value.listener_id;
    if (buildForm.value.server_addr) data.server_addr = buildForm.value.server_addr;
    if (buildForm.value.agent_token) data.agent_token = buildForm.value.agent_token;
    if (buildForm.value.heartbeat_secs) data.heartbeat_secs = buildForm.value.heartbeat_secs;
    if (buildForm.value.jitter) data.jitter = buildForm.value.jitter;

    await createAgentBuild(data);
    message.success('构建任务已提交');
    buildModalVisible.value = false;
    resetBuildForm();
    loadBuilds();
  } catch (err: any) {
    message.error(err.message || '创建构建失败');
  } finally {
    building.value = false;
  }
};

// Log drawer functions
const openBuildLog = (record: AgentBuildRecord) => {
  logBuildId.value = record.build_id;
  logBuildStatus.value = record.status;
  logBuildDetail.value = record.detail || '';
  logDrawerVisible.value = true;
};

const resetBuildForm = () => {
  buildForm.value = {
    target_triple: undefined,
    listener_id: undefined,
    server_addr: '',
    agent_token: '',
    profile: 'release',
    heartbeat_secs: undefined,
    jitter: undefined,
  };
};

// Formatting Helpers
const formatTimestamp = (ts: number | null | undefined) => {
  if (!ts) return '-';
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
  if (status === 'succeeded') return 'bg-green-500';
  if (status === 'pending') return 'bg-blue-400';
  if (status === 'failed') return 'bg-red-500';
  return 'bg-slate-400';
};

const getStatusTextColor = (status: string) => {
  if (status === 'succeeded') return 'text-green-600 font-medium';
  if (status === 'pending') return 'text-blue-600 font-medium';
  if (status === 'failed') return 'text-red-600 font-medium';
  return 'text-slate-500';
};

const getStatusLabel = (status: string) => {
  if (status === 'succeeded') return '成功';
  if (status === 'pending') return '构建中';
  if (status === 'failed') return '失败';
  return status;
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
:deep(.ant-table-thead > tr > th) {
  padding: 8px 12px !important;
  line-height: 1.4;
}
:deep(.ant-table-measure-row) {
  display: none !important;
}
:deep(.ant-table-tbody > tr > td) {
  padding: 8px 12px !important;
  line-height: 1.4;
}
:deep(.ant-table-body) {
  flex-grow: 1;
  overflow-y: auto !important;
}
</style>
