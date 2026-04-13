<template>
  <a-drawer
    :open="visible"
    :title="`节点详情: ${localAgent?.agent_id || ''}`"
    placement="right"
    width="600"
    @update:open="$emit('update:visible', $event)"
    @close="onClose"
  >
    <div v-if="localAgent" class="space-y-6">
      <!-- Status Banner -->
      <div class="p-4 rounded-lg flex items-center justify-between border" 
           :class="localAgent.is_online ? 'bg-green-50/50 border-green-200 dark:bg-green-900/10 dark:border-green-900/30' : 'bg-slate-50 border-slate-200 dark:bg-[#14161A] dark:border-slate-800'">
        <div class="flex items-center gap-3">
          <div class="w-3 h-3 rounded-full" :class="localAgent.is_online ? 'bg-green-500' : 'bg-slate-400'"></div>
          <span class="font-medium" :class="localAgent.is_online ? 'text-green-700 dark:text-green-400' : 'text-slate-600 dark:text-slate-400'">
            {{ localAgent.is_online ? '在线 (Online)' : '离线 (Offline)' }}
          </span>
          <a-tag v-if="localAgent.is_disabled" color="error">已禁用</a-tag>
        </div>
        <div class="text-xs text-slate-500">
          最后活跃: {{ formatTimestamp(localAgent.last_seen) }}
        </div>
      </div>

      <a-descriptions title="基础信息" :column="2" bordered size="small" class="bg-white dark:bg-[#1C1E22]">
        <a-descriptions-item label="主机名">{{ localAgent.hostname }}</a-descriptions-item>
        <a-descriptions-item label="用户名">{{ localAgent.username }}</a-descriptions-item>
        <a-descriptions-item label="操作系统">{{ localAgent.os }}</a-descriptions-item>
        <a-descriptions-item label="架构">{{ localAgent.arch }}</a-descriptions-item>
        <a-descriptions-item label="进程 ID">{{ localAgent.pid }}</a-descriptions-item>
        <a-descriptions-item label="会话 ID">{{ localAgent.session_id !== null ? localAgent.session_id : '-' }}</a-descriptions-item>
        <a-descriptions-item label="标签" :span="2">
          <div class="flex items-center gap-1 flex-wrap">
            <a-tag v-for="tag in localAgent.tags" :key="tag" color="blue" closable @close="removeTag(tag)">{{ tag }}</a-tag>
            <a-input
              v-if="tagInputVisible"
              ref="tagInputRef"
              v-model:value="tagInputValue"
              size="small"
              style="width: 100px"
              placeholder="输入标签"
              @blur="handleTagInputConfirm"
              @keyup.enter="handleTagInputConfirm"
            />
            <a-button v-else size="small" type="dashed" @click="showTagInput">
              <template #icon><PlusOutlined /></template> 添加
            </a-button>
          </div>
        </a-descriptions-item>
      </a-descriptions>

      <a-descriptions title="网络配置" :column="2" bordered size="small" class="bg-white dark:bg-[#1C1E22]">
        <a-descriptions-item label="内部 IP">{{ localAgent.internal_ip }}</a-descriptions-item>
        <a-descriptions-item label="外部 IP">{{ localAgent.external_ip }}</a-descriptions-item>
        <a-descriptions-item label="对端地址" :span="2">{{ localAgent.peer_addr }}</a-descriptions-item>
      </a-descriptions>

      <div class="border border-gray-200 dark:border-[#14161A] rounded-lg overflow-hidden">
        <div class="bg-slate-50 dark:bg-[#14161A] px-4 py-2 border-b border-gray-200 dark:border-[#14161A] font-medium text-slate-800 dark:text-slate-200">
          Beacon 通信配置
        </div>
        <div class="p-4 bg-white dark:bg-[#1C1E22]">
          <a-form layout="vertical" class="flex gap-4">
            <a-form-item label="休眠间隔 (秒)" class="flex-1 mb-0">
              <a-input-number v-model:value="beaconForm.sleep_interval" :min="1" class="w-full" :disabled="!localAgent.is_online || localAgent.is_disabled" />
            </a-form-item>
            <a-form-item label="抖动 (Jitter %)" class="flex-1 mb-0">
              <a-input-number v-model:value="beaconForm.jitter" :min="0" :max="100" class="w-full" :disabled="!localAgent.is_online || localAgent.is_disabled" />
            </a-form-item>
            <div class="flex items-end mb-0">
              <a-button type="primary" @click="handleUpdateBeacon" :loading="beaconUpdating" :disabled="!localAgent.is_online || localAgent.is_disabled">
                应用
              </a-button>
            </div>
          </a-form>
          <div v-if="!localAgent.is_online || localAgent.is_disabled" class="mt-2 text-xs text-orange-500">
            * 节点当前离线或被禁用，无法更新 Beacon 配置。
          </div>
        </div>
      </div>

      <div class="flex gap-2 justify-end pt-4 border-t border-gray-200 dark:border-[#14161A]">
        <a-button type="primary" :disabled="localAgent.is_disabled" @click="openTaskModal">下发任务</a-button>
        
        <a-dropdown placement="topRight">
          <a-button>更多管理操作 <DownOutlined /></a-button>
          <template #overlay>
            <a-menu>
              <a-menu-item key="disconnect" :disabled="!localAgent.is_online" @click="handleAction('disconnect')">断开连接</a-menu-item>
              <a-menu-item key="disable" v-if="!localAgent.is_disabled" @click="handleAction('disable')">禁用节点</a-menu-item>
              <a-menu-item key="enable" v-if="localAgent.is_disabled" @click="handleAction('enable')">启用节点</a-menu-item>
              <a-menu-divider />
              <a-menu-item key="delete" :disabled="localAgent.is_online" danger @click="handleAction('delete')">删除记录</a-menu-item>
            </a-menu>
          </template>
        </a-dropdown>
      </div>

    </div>
    <div v-else class="flex justify-center py-10">
      <a-spin />
    </div>
  </a-drawer>
</template>

<script setup lang="ts">
import { ref, reactive, watch, nextTick } from 'vue';
import { message } from 'ant-design-vue';
import { DownOutlined, PlusOutlined } from '@ant-design/icons-vue';
import { fetchAgentDetail, updateBeaconConfig, updateAgentTags } from '@/api/agent';
import type { Agent } from '@/api/agent';
import { formatTimestamp } from '@/utils/format';

const props = defineProps<{
  visible: boolean;
  agent: Agent | null;
}>();

const emit = defineEmits(['update:visible', 'open-task', 'action', 'update:agent']);

const localAgent = ref<Agent | null>(null);
const beaconUpdating = ref(false);
const beaconForm = reactive({
  sleep_interval: 10,
  jitter: 20
});
const tagInputVisible = ref(false);
const tagInputValue = ref('');
const tagInputRef = ref<any>(null);

watch(() => props.visible, async (newVal) => {
  if (newVal && props.agent) {
    localAgent.value = { ...props.agent };
    beaconForm.sleep_interval = props.agent.sleep_interval;
    beaconForm.jitter = props.agent.jitter;
    try {
      const freshData = await fetchAgentDetail(props.agent.agent_id);
      localAgent.value = freshData;
      emit('update:agent', freshData);
      beaconForm.sleep_interval = freshData.sleep_interval;
      beaconForm.jitter = freshData.jitter;
    } catch (e: any) {
      if (e.message && !e.message.includes('canceled')) {
        message.warning('同步最新状态失败: ' + e.message);
      }
    }
  }
});

async function handleUpdateBeacon() {
  if (!localAgent.value) return;
  beaconUpdating.value = true;
  try {
    const res = await updateBeaconConfig(localAgent.value.agent_id, beaconForm.sleep_interval, beaconForm.jitter);
    if (res.success) {
      message.success('Beacon 配置更新成功');
      localAgent.value = res.agent;
      emit('update:agent', res.agent);
    }
  } catch (e: any) {
    message.error(e.message);
  } finally {
    beaconUpdating.value = false;
  }
}

function handleAction(action: string) {
  if (localAgent.value) {
    emit('action', { action, agent: localAgent.value });
  }
}

function showTagInput() {
  tagInputVisible.value = true;
  tagInputValue.value = '';
  nextTick(() => {
    tagInputRef.value?.focus();
  });
}

async function handleTagInputConfirm() {
  tagInputVisible.value = false;
  const newTag = tagInputValue.value.trim();
  if (!newTag || !localAgent.value) return;

  const tags = [...(localAgent.value.tags || [])];
  if (tags.includes(newTag)) return;
  tags.push(newTag);

  await saveTags(tags);
}

async function removeTag(tag: string) {
  if (!localAgent.value) return;
  const tags = (localAgent.value.tags || []).filter(t => t !== tag);
  await saveTags(tags);
}

async function saveTags(tags: string[]) {
  if (!localAgent.value) return;
  try {
    const res = await updateAgentTags(localAgent.value.agent_id, tags);
    if (res.success) {
      localAgent.value = { ...localAgent.value, tags };
      emit('update:agent', localAgent.value);
      message.success('标签已更新');
    }
  } catch (e: any) {
    message.error(e.message);
  }
}

function openTaskModal() {
  if (localAgent.value) {
    emit('open-task', localAgent.value);
  }
}

function onClose() {
  emit('update:visible', false);
}
</script>
