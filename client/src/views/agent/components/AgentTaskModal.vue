<template>
  <a-modal :open="visible" :title="`快速下发任务至 ${agent?.agent_id || ''}`" @ok="submitTask" :confirmLoading="taskSubmitting" @update:open="$emit('update:visible', $event)">
    <a-form layout="vertical" class="mt-4">
      <a-form-item label="快捷命令模板">
        <div class="flex gap-2 mb-2">
          <a-tag color="blue" class="cursor-pointer" @click="taskForm.cmd = 'whoami'; taskForm.args = ''">whoami</a-tag>
          <a-tag color="blue" class="cursor-pointer" @click="taskForm.cmd = 'hostname'; taskForm.args = ''">hostname</a-tag>
          <a-tag color="blue" class="cursor-pointer" @click="taskForm.cmd = 'ipconfig'; taskForm.args = ''">ipconfig</a-tag>
        </div>
      </a-form-item>
      <a-form-item label="执行命令 (Command)" required>
        <a-input v-model:value="taskForm.cmd" placeholder="例如: whoami" />
      </a-form-item>
      <a-form-item label="参数 (Args)">
        <a-input v-model:value="taskForm.args" placeholder="以空格分隔的参数列表" />
      </a-form-item>
    </a-form>
  </a-modal>
</template>

<script setup lang="ts">
import { ref, reactive, watch } from 'vue';
import { message } from '@/utils/message';
import { dispatchTask } from '@/api/agent';
import type { Agent } from '@/api/agent';

const props = defineProps<{
  visible: boolean;
  agent: Agent | null;
}>();

const emit = defineEmits(['update:visible']);

const taskSubmitting = ref(false);
const taskForm = reactive({ cmd: '', args: '' });

watch(() => props.visible, (newVal) => {
  if (newVal) {
    taskForm.cmd = '';
    taskForm.args = '';
  }
});

async function submitTask() {
  if (!props.agent || !taskForm.cmd) {
    message.warning('请填写完整的命令');
    return;
  }
  taskSubmitting.value = true;
  try {
    const argsArray = taskForm.args.trim().split(/\s+/).filter(Boolean);
    const payload = { kind: 'shell', cmd: taskForm.cmd, args: argsArray };
    await dispatchTask(props.agent.agent_id, payload);
    message.success('任务下发成功');
    emit('update:visible', false);
  } catch (e: any) {
    message.error(e.message);
  } finally {
    taskSubmitting.value = false;
  }
}
</script>
