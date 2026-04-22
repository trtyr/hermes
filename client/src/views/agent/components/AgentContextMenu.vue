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
        class="px-3 py-2 hover:bg-blue-50 cursor-pointer flex items-center gap-2 text-sm text-slate-700"
        :class="{ 'opacity-50 cursor-not-allowed': agent?.is_disabled }"
        @click="!agent?.is_disabled && emitAction('open-task')"
      >
        <CodeOutlined /> 快捷命令 (弹窗)
      </div>
      
      <div 
        class="px-3 py-2 hover:bg-blue-50 cursor-pointer flex items-center gap-2 text-sm text-slate-700"
        :class="{ 'opacity-50 cursor-not-allowed': agent?.is_disabled }"
        @click="!agent?.is_disabled && emitAction('open-terminal')"
      >
        <DesktopOutlined /> 打开独立终端
      </div>
      
      <div 
        class="px-3 py-2 hover:bg-blue-50 cursor-pointer flex items-center gap-2 text-sm text-slate-700"
        :class="{ 'opacity-50 cursor-not-allowed': agent?.is_disabled }"
        @click="!agent?.is_disabled && emitAction('open-file-ops')"
      >
        <FolderOpenOutlined /> 文件管理
      </div>
      
      <div 
        class="px-3 py-2 hover:bg-blue-50 cursor-pointer flex items-center gap-2 text-sm text-slate-700"
        :class="{ 'opacity-50 cursor-not-allowed': agent?.is_disabled }"
        @click="!agent?.is_disabled && emitAction('screenshot')"
      >
        <CameraOutlined /> 屏幕截图
      </div>

      <div 
        class="px-3 py-2 hover:bg-blue-50 cursor-pointer flex items-center gap-2 text-sm text-slate-700"
        :class="{ 'opacity-50 cursor-not-allowed': agent?.is_disabled }"
        @click="!agent?.is_disabled && emitAction('ps')"
      >
        <UnorderedListOutlined /> 进程列表
      </div>

      <div class="h-[1px] bg-gray-100 my-1"></div>

      <div 
        v-if="!agent?.is_disabled"
        class="px-3 py-2 hover:bg-orange-50 cursor-pointer flex items-center gap-2 text-sm text-orange-600"
        @click="agent && emitAction('action', 'disable')"
      >
        <StopOutlined /> 禁用节点
      </div>
      
      <div 
        v-if="agent?.is_disabled"
        class="px-3 py-2 hover:bg-green-50 cursor-pointer flex items-center gap-2 text-sm text-green-600"
        @click="agent && emitAction('action', 'enable')"
      >
        <CheckCircleOutlined /> 启用节点
      </div>

      <div 
        v-if="agent?.is_online"
        class="px-3 py-2 hover:bg-red-50 cursor-pointer flex items-center gap-2 text-sm text-red-600"
        @click="agent && emitAction('action', 'disconnect')"
      >
        <DisconnectOutlined /> 断开连接
      </div>

      <div 
        v-if="!agent?.is_online"
        class="px-3 py-2 hover:bg-red-50 cursor-pointer flex items-center gap-2 text-sm text-red-600"
        @click="agent && emitAction('action', 'delete')"
      >
        <DeleteOutlined /> 删除记录
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { 
  DesktopOutlined, DisconnectOutlined, StopOutlined, CheckCircleOutlined,
  DeleteOutlined, CodeOutlined, FolderOpenOutlined, CameraOutlined, UnorderedListOutlined
} from '@ant-design/icons-vue';
import type { Agent } from '@/api/agent';

const props = defineProps<{
  visible: boolean;
  x: number;
  y: number;
  agent: Agent | null;
}>();

const emit = defineEmits(['action', 'open-task', 'open-terminal', 'open-file-ops', 'screenshot', 'ps', 'close']);

function emitAction(type: 'action' | 'open-task' | 'open-terminal' | 'open-file-ops' | 'screenshot' | 'ps', act?: string) {
  if (type === 'action') {
    emit('action', { action: act, agent: props.agent });
  } else if (type === 'open-task') {
    emit('open-task', props.agent);
  } else if (type === 'open-file-ops') {
    emit('open-file-ops', props.agent);
  } else if (type === 'screenshot') {
    emit('screenshot', props.agent);
  } else if (type === 'ps') {
    emit('ps', props.agent);
  } else {
    emit('open-terminal', props.agent);
  }
  emit('close');
}
</script>
