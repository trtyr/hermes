<template>
  <div class="h-full w-full flex flex-col p-4 relative">
    <!-- Header Actions -->
    <div class="flex justify-between items-center mb-4">
      <h2 class="text-xl font-semibold text-slate-800">Agent管理</h2>
      <div class="flex items-center gap-2">
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
        @change="handleTableChange"
        size="middle"
        :scroll="{ x: 'max-content', y: 'calc(100vh - 280px)' }"
        rowKey="agent_id"
        class="w-full h-full"
      >
        <template #bodyCell="{ column, record }">
          <template v-if="column.key === 'agent_id'">
            <a-button type="link" class="p-0 font-medium" @click="openDetail(record)">
              {{ record.agent_id }}
            </a-button>
          </template>

          <template v-else-if="column.key === 'status'">
            <div class="flex flex-col gap-1">
              <a-badge :status="record.is_online ? 'success' : 'default'" :text="record.is_online ? '在线' : '离线'" />
              <a-badge v-if="record.is_disabled" status="error" text="已禁用" />
            </div>
          </template>

          <template v-else-if="column.key === 'platform'">
            <div class="flex items-center gap-1.5">
              <WindowsOutlined v-if="record.os && record.os.toLowerCase().includes('windows')" class="text-blue-500" />
              <AppleOutlined v-else-if="record.os && (record.os.toLowerCase().includes('mac') || record.os.toLowerCase().includes('darwin'))" class="text-gray-500" />
              <svg v-else-if="record.os && record.os.toLowerCase().includes('linux')" class="w-3.5 h-3.5 text-yellow-600" viewBox="0 0 24 24" fill="currentColor"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm-1-13h2v6h-2zm0 8h2v2h-2z"/></svg>
              <DesktopOutlined v-else class="text-slate-500" />
              <span>{{ record.os }} / {{ record.arch }}</span>
            </div>
          </template>

          <template v-else-if="column.key === 'network'">
            <div class="text-xs space-y-0.5">
              <div><span class="text-slate-400">内部:</span> {{ record.internal_ip }}</div>
              <div><span class="text-slate-400">外部:</span> {{ record.external_ip }}</div>
            </div>
          </template>

          <template v-else-if="column.key === 'beacon'">
            <div class="text-xs">{{ record.sleep_interval }}s ±{{ record.jitter }}%</div>
          </template>

          <template v-else-if="column.key === 'last_seen'">
            <span class="text-xs">{{ formatTimestamp(record.last_seen) }}</span>
          </template>

          <template v-else-if="column.key === 'action'">
            <div class="flex items-center gap-2">
              <a-button type="link" size="small" class="p-0" @click="openDetail(record)">管理</a-button>
              
              <a-dropdown :trigger="['click']">
                <a-button type="text" size="small" class="px-1"><MoreOutlined /></a-button>
                <template #overlay>
                  <a-menu>
                    <a-menu-item key="task" :disabled="record.is_disabled" @click="openTaskModal(record)">
                      <template #icon><CodeOutlined /></template> 快捷命令 (弹窗)
                    </a-menu-item>
                    <a-menu-item key="terminal" :disabled="record.is_disabled" @click="openTerminal(record)">
                      <template #icon><DesktopOutlined /></template> 打开独立终端
                    </a-menu-item>
                    <a-menu-item key="fileops" :disabled="record.is_disabled" @click="openFileOps(record)">
                      <template #icon><FolderOpenOutlined /></template> 文件管理
                    </a-menu-item>
                    <a-menu-divider />
                    <a-menu-item key="disconnect" :disabled="!record.is_online" @click="handleAction({ action: 'disconnect', agent: record })">
                      <template #icon><DisconnectOutlined /></template> 断开连接
                    </a-menu-item>
                    <a-menu-item key="disable" v-if="!record.is_disabled" @click="handleAction({ action: 'disable', agent: record })">
                      <template #icon><StopOutlined /></template> 禁用节点
                    </a-menu-item>
                    <a-menu-item key="enable" v-if="record.is_disabled" @click="handleAction({ action: 'enable', agent: record })">
                      <template #icon><CheckCircleOutlined /></template> 启用节点
                    </a-menu-item>
                    <a-menu-divider />
                    <a-menu-item key="delete" :disabled="record.is_online" danger @click="handleAction({ action: 'delete', agent: record })">
                      <template #icon><DeleteOutlined /></template> 删除记录
                    </a-menu-item>
                  </a-menu>
                </template>
              </a-dropdown>
            </div>
          </template>
        </template>
      </a-table>
    </div>

    <!-- Extracted Child Components -->
    <AgentDetailDrawer 
      v-model:visible="detailVisible" 
      :agent="selectedAgent"
      @update:agent="handleAgentStoreUpdate"
      @open-task="openTaskModal" 
      @action="handleAction"
    />

    <AgentTaskModal 
      v-model:visible="taskModalVisible" 
      :agent="actionAgent" 
    />

    <FileOpsModal 
      v-model:visible="fileOpsVisible" 
      :agent="actionAgent" 
    />

    <AgentContextMenu 
      :visible="contextMenuState.visible" 
      :x="contextMenuState.x" 
      :y="contextMenuState.y" 
      :agent="contextMenuState.record"
      @close="contextMenuState.visible = false"
      @open-task="openTaskModal"
      @open-terminal="openTerminal"
      @open-file-ops="openFileOps"
      @screenshot="handleScreenshot"
      @ps="handlePs"
      @action="handleAction"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, onBeforeUnmount } from 'vue';
import { useRouter } from 'vue-router';
import { message, Modal } from 'ant-design-vue';
import { ReloadOutlined, WindowsOutlined, AppleOutlined, DesktopOutlined, MoreOutlined, CodeOutlined, DisconnectOutlined, StopOutlined, CheckCircleOutlined, DeleteOutlined, FolderOpenOutlined } from '@ant-design/icons-vue';
import { fetchAgents, disconnectAgent, disableAgent, enableAgent, deleteAgent, fetchAgentDetail, takeScreenshot, dispatchTask } from '@/api/agent';
import type { Agent } from '@/api/agent';
import { formatTimestamp } from '@/utils/format';
import { useAgentWebSocket } from './hooks/useAgentWebSocket';
import AgentDetailDrawer from './components/AgentDetailDrawer.vue';
import AgentTaskModal from './components/AgentTaskModal.vue';
import FileOpsModal from './components/FileOpsModal.vue';
import AgentContextMenu from './components/AgentContextMenu.vue';

const router = useRouter();

// Core State
const agents = ref<Agent[]>([]);
const loading = ref(false);
const searchKeyword = ref('');
const pagination = reactive({ current: 1, pageSize: 20, total: 0, showSizeChanger: true });

// UI Component State
const detailVisible = ref(false);
const taskModalVisible = ref(false);
const fileOpsVisible = ref(false);
const selectedAgent = ref<Agent | null>(null);
const actionAgent = ref<Agent | null>(null); 
const contextMenuState = reactive({ visible: false, x: 0, y: 0, record: null as Agent | null });

const columns = [
  { title: '节点 ID', dataIndex: 'agent_id', key: 'agent_id', width: 140 },
  { title: '状态', key: 'status', width: 100 },
  { title: '用户名', dataIndex: 'username', key: 'username', width: 120 },
  { title: '平台/架构', key: 'platform', width: 180 },
  { title: '网络地址', key: 'network', width: 160 },
  { title: 'Beacon', key: 'beacon', width: 120 },
  { title: '最后活跃', key: 'last_seen', width: 160 },
  { title: '操作', key: 'action', width: 100, fixed: 'right' }
];

// Initialize WebSocket Hook (Microkernel approach)
useAgentWebSocket(agents, selectedAgent, detailVisible, loadAgents);

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

function openDetail(agent: Agent) {
  selectedAgent.value = agent;
  detailVisible.value = true;
}

function openTaskModal(agent: Agent) {
  actionAgent.value = agent;
  taskModalVisible.value = true;
}

function openFileOps(agent: Agent) {
  actionAgent.value = agent;
  fileOpsVisible.value = true;
}

async function handleScreenshot(agent: Agent) {
  try {
    const res = await takeScreenshot(agent.agent_id);
    if (res.success) {
      message.success(`截图任务已下发 (task: ${res.task_id})`);
    } else {
      message.error(res.detail || '截图失败');
    }
  } catch (e: any) {
    message.error(e.message);
  }
}

async function handlePs(agent: Agent) {
  try {
    const res = await dispatchTask(agent.agent_id, { command: 'ps' });
    if (res.success) {
      message.success(`进程列表任务已下发 (task: ${res.task_id})`);
    } else {
      message.error(res.detail || '获取进程列表失败');
    }
  } catch (e: any) {
    message.error(e.message);
  }
}

function openTerminal(agent: Agent) {
  router.push(`/agent/terminal/${agent.agent_id}`);
}

function handleAgentStoreUpdate(agent: Agent) {
  // Sync changes back to the main agents array if needed
  const idx = agents.value.findIndex(a => a.agent_id === agent.agent_id);
  if (idx > -1) agents.value[idx] = agent;
  if (selectedAgent.value?.agent_id === agent.agent_id) {
    selectedAgent.value = agent;
  }
}

function handleAction({ action, agent }: { action: string, agent: Agent }) {
  const actionMap: Record<string, { title: string, func: Function }> = {
    'disconnect': { title: '断开连接', func: disconnectAgent },
    'disable': { title: '禁用节点', func: disableAgent },
    'enable': { title: '启用节点', func: enableAgent },
    'delete': { title: '删除记录', func: deleteAgent },
  };
  
  const target = actionMap[action];
  if (!target) return;

  Modal.confirm({
    title: `确认要对节点 ${agent.agent_id} 执行 [${target.title}] 操作吗？`,
    content: action === 'delete' ? '删除后将不可恢复。' : '',
    okType: action === 'delete' || action === 'disable' || action === 'disconnect' ? 'danger' : 'primary',
    async onOk() {
      try {
        await target.func(agent.agent_id);
        message.success(`操作 [${target.title}] 执行成功`);
        
        if (action === 'delete') {
          detailVisible.value = false;
        } else if (detailVisible.value && selectedAgent.value?.agent_id === agent.agent_id) {
          selectedAgent.value = await fetchAgentDetail(agent.agent_id);
        }
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
