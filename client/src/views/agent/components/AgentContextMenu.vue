<template>
  <Teleport to="body">
    <div
      v-if="visible"
      class="fixed z-50 bg-white border border-gray-200 rounded-md shadow-lg py-1 min-w-[160px]"
      :style="{ top: `${y}px`, left: `${x}px` }"
      @click.stop
    >
      <div class="px-3 py-1.5 text-xs text-slate-400 border-b border-gray-100 mb-1">
        Agent: {{ agent?.agent_id }}
      </div>

      <div
        v-if="agent?.is_online"
        class="px-3 py-2 hover:bg-red-50 cursor-pointer flex items-center gap-2 text-sm text-red-600"
        @click="agent && emitAction('disconnect')"
      >
        <DisconnectOutlined /> 断开连接
      </div>

      <div
        v-if="!agent?.is_disabled"
        class="px-3 py-2 hover:bg-orange-50 cursor-pointer flex items-center gap-2 text-sm text-orange-600"
        @click="agent && emitAction('disable')"
      >
        <StopOutlined /> 禁用节点
      </div>

      <div
        v-if="agent?.is_disabled"
        class="px-3 py-2 hover:bg-green-50 cursor-pointer flex items-center gap-2 text-sm text-green-600"
        @click="agent && emitAction('enable')"
      >
        <CheckCircleOutlined /> 启用节点
      </div>

      <div class="h-[1px] bg-gray-100 my-1"></div>

      <div
        v-if="!agent?.is_online"
        class="px-3 py-2 hover:bg-red-50 cursor-pointer flex items-center gap-2 text-sm text-red-600"
        @click="agent && emitAction('delete')"
      >
        <DeleteOutlined /> 删除记录
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import {
  DisconnectOutlined, StopOutlined, CheckCircleOutlined, DeleteOutlined
} from '@ant-design/icons-vue';
import type { Agent } from '@/api/agent';

const props = defineProps<{
  visible: boolean;
  x: number;
  y: number;
  agent: Agent | null;
}>();

const emit = defineEmits(['action', 'close']);

function emitAction(act: string) {
  emit('action', { action: act, agent: props.agent });
  emit('close');
}
</script>
