<template>
  <div class="h-full w-full flex flex-col p-4 relative">
    <!-- Header -->
    <div class="flex justify-between items-center mb-4">
      <h2 class="text-xl font-semibold text-slate-800 dark:text-[var(--text-primary)] flex items-center gap-2 m-0">
        <RocketOutlined class="text-orange-500" />
        载荷生成
      </h2>
      <div class="flex gap-2">
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
    <div class="flex-1 bg-white dark:bg-[var(--bg-card)] rounded-lg border border-gray-200 dark:border-[var(--border-default)] shadow-sm flex flex-col overflow-hidden">
      <a-table
        :columns="columns"
        :data-source="builds"
        row-key="build_id"
        :loading="loading"
        :pagination="{ pageSize: 20, total: total, current: currentPage, onChange: onPageChange }"
        class="w-full flex-1"
        :scroll="{ y: 'max-content' }"
      >
        <!-- Custom Body Cells -->
        <template #bodyCell="{ column, record }">
          <template v-if="column.key === 'build_id'">
            <span class="font-mono text-sm">#{{ record.build_id }}</span>
          </template>

          <template v-else-if="column.key === 'target_triple'">
            <span class="font-mono text-sm text-slate-600 dark:text-[var(--text-secondary)]">
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
            <span v-if="record.listener_id" class="font-mono text-sm">
              #{{ record.listener_id }}
            </span>
            <span v-else class="text-slate-400">-</span>
          </template>

          <template v-else-if="column.key === 'server_addr'">
            <span class="font-mono text-sm text-slate-600 dark:text-[var(--text-secondary)]">
              {{ record.server_addr || '-' }}
            </span>
          </template>

          <template v-else-if="column.key === 'created_at'">
            {{ formatTimestamp(record.created_at) }}
          </template>

          <template v-else-if="column.key === 'detail'">
            <a-tooltip v-if="record.detail" :title="record.detail">
              <span class="text-sm text-slate-500 dark:text-[var(--text-secondary)] truncate max-w-[200px] inline-block align-bottom">
                {{ record.detail }}
              </span>
            </a-tooltip>
            <span v-else class="text-slate-400">-</span>
          </template>

          <template v-else-if="column.key === 'action'">
            <a-button
              v-if="record.status === 'succeeded'"
              type="link"
              size="small"
              @click="handleDownload(record)"
            >
              <template #icon><DownloadOutlined /></template>
              下载
            </a-button>
            <span v-else class="text-slate-400 text-xs">-</span>
          </template>
        </template>
      </a-table>
    </div>

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
        <a-form-item label="目标平台" help="留空则使用当前服务器平台">
          <a-select
            v-model:value="buildForm.target_triple"
            allowClear
            placeholder="自动检测当前平台"
          >
            <a-select-option value="x86_64-pc-windows-msvc">Windows x86_64 (MSVC)</a-select-option>
            <a-select-option value="i686-pc-windows-msvc">Windows i686 (MSVC)</a-select-option>
            <a-select-option value="x86_64-unknown-linux-gnu">Linux x86_64 (GNU)</a-select-option>
            <a-select-option value="aarch64-unknown-linux-gnu">Linux ARM64 (GNU)</a-select-option>
            <a-select-option value="x86_64-apple-darwin">macOS x86_64</a-select-option>
            <a-select-option value="aarch64-apple-darwin">macOS ARM64</a-select-option>
          </a-select>
        </a-form-item>

        <a-form-item label="绑定监听器" help="推荐选择，用于确定通信协议和回连地址">
          <a-select
            v-model:value="buildForm.listener_id"
            allowClear
            placeholder="选择监听器"
            :loading="listenersLoading"
          >
            <a-select-option v-for="l in listeners" :key="l.id" :value="l.id">
              <span class="font-mono mr-2">#{{ l.id }}</span>
              {{ l.name }}
              <a-tag :color="getProtocolColor(l.protocol)" size="small" class="ml-2 mr-0">
                {{ l.protocol }}
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

        <a-form-item label="构建配置">
          <a-radio-group v-model:value="buildForm.profile">
            <a-radio-button value="release">Release（体积最小）</a-radio-button>
            <a-radio-button value="debug">Debug（带调试信息）</a-radio-button>
          </a-radio-group>
        </a-form-item>
      </a-form>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { message } from 'ant-design-vue';
import {
  RocketOutlined,
  ReloadOutlined,
  PlusOutlined,
  DownloadOutlined,
} from '@ant-design/icons-vue';
import dayjs from 'dayjs';

import {
  AgentBuildRecord,
  fetchAgentBuilds,
  createAgentBuild,
  getBuildDownloadUrl,
} from '@/api/agentBuild';
import { fetchListeners, ListenerRecord } from '@/api/listener';
import { useConnectionStore } from '@/store/connection';

const builds = ref<AgentBuildRecord[]>([]);
const loading = ref(false);
const total = ref(0);
const currentPage = ref(1);
const pageSize = 20;

// Build form
const buildModalVisible = ref(false);
const building = ref(false);
const buildForm = ref({
  target_triple: undefined as string | undefined,
  listener_id: undefined as number | undefined,
  server_addr: '',
  agent_token: '',
  profile: 'release',
});

// Listeners for the form dropdown
const listeners = ref<ListenerRecord[]>([]);
const listenersLoading = ref(false);

const columns = [
  { title: '构建 ID', dataIndex: 'build_id', key: 'build_id', width: 100 },
  { title: '目标平台', dataIndex: 'target_triple', key: 'target_triple', width: 220 },
  { title: '配置', dataIndex: 'profile', key: 'profile', width: 90 },
  { title: '监听器', dataIndex: 'listener_id', key: 'listener_id', width: 100 },
  { title: '回连地址', dataIndex: 'server_addr', key: 'server_addr', width: 180 },
  { title: '状态', dataIndex: 'status', key: 'status', width: 120 },
  { title: '详情', dataIndex: 'detail', key: 'detail', width: 200, ellipsis: true },
  { title: '创建时间', dataIndex: 'created_at', key: 'created_at', width: 170 },
  { title: '操作', key: 'action', width: 80, fixed: 'right' },
];

const loadBuilds = async () => {
  loading.value = true;
  try {
    const res = await fetchAgentBuilds({
      limit: pageSize,
      offset: (currentPage.value - 1) * pageSize,
    });
    builds.value = res.builds || [];
    total.value = res.total || 0;
  } catch (err: any) {
    message.error(err.message || '获取构建列表失败');
  } finally {
    loading.value = false;
  }
};

async function handleDownload(record: AgentBuildRecord) {
  const store = useConnectionStore();
  const profile = store.activeProfile;
  if (!profile) return;
  try {
    const url = getBuildDownloadUrl(record.build_id);
    const res = await fetch(url, {
      headers: { 'Authorization': `Bearer ${profile.api_token}` }
    });
    if (!res.ok) throw new Error('下载失败');
    const blob = await res.blob();
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

onMounted(() => {
  loadBuilds();
  loadListeners();
});

// Build action
const handleBuild = async () => {
  building.value = true;
  try {
    const data: Record<string, any> = {
      profile: buildForm.value.profile,
    };
    if (buildForm.value.target_triple) data.target_triple = buildForm.value.target_triple;
    if (buildForm.value.listener_id) data.listener_id = buildForm.value.listener_id;
    if (buildForm.value.server_addr) data.server_addr = buildForm.value.server_addr;
    if (buildForm.value.agent_token) data.agent_token = buildForm.value.agent_token;

    await createAgentBuild(data);
    message.success('构建任务已提交');
    buildModalVisible.value = false;
    resetBuildForm();
    // Reload after a short delay so pending builds show up
    setTimeout(() => loadBuilds(), 500);
  } catch (err: any) {
    message.error(err.message || '创建构建失败');
  } finally {
    building.value = false;
  }
};

const resetBuildForm = () => {
  buildForm.value = {
    target_triple: undefined,
    listener_id: undefined,
    server_addr: '',
    agent_token: '',
    profile: 'release',
  };
};

// Formatting Helpers
const formatTimestamp = (ts: number | null | undefined) => {
  if (!ts) return '-';
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
  if (status === 'succeeded') return 'bg-green-500';
  if (status === 'pending') return 'bg-blue-400';
  if (status === 'failed') return 'bg-red-500';
  return 'bg-slate-400';
};

const getStatusTextColor = (status: string) => {
  if (status === 'succeeded') return 'text-green-600 dark:text-green-500 font-medium';
  if (status === 'pending') return 'text-blue-600 dark:text-blue-400 font-medium';
  if (status === 'failed') return 'text-red-600 dark:text-red-500 font-medium';
  return 'text-slate-500 dark:text-[var(--text-secondary)]';
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
}
:deep(.ant-table-body) {
  overflow-y: auto !important;
}
</style>
