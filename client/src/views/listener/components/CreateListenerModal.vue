<template>
  <a-modal
    :open="visible"
    title="新建监听器 (Spawn Listener)"
    @update:open="$emit('update:visible', $event)"
    @cancel="handleCancel"
    @ok="handleSubmit"
    :confirmLoading="submitting"
    destroyOnClose
  >
    <a-form :model="formState" :rules="rules" ref="formRef" layout="vertical" class="mt-4">
      <a-form-item label="监听器名称 (Name)" name="name">
        <a-input v-model:value="formState.name" placeholder="请输入标识名称，如 HTTPS Beacon US-East" />
      </a-form-item>
      
      <a-form-item label="协议类型 (Protocol)" name="kind">
        <a-select v-model:value="formState.kind" placeholder="选择通信协议">
          <a-select-option value="tcp_json">TCP</a-select-option>
          <a-select-option value="https_json">HTTPS</a-select-option>
        </a-select>
      </a-form-item>

      <div class="flex gap-4">
        <a-form-item label="绑定地址 (Bind Host)" name="bind_host" class="flex-1">
          <a-input v-model:value="formState.bind_host" placeholder="0.0.0.0" />
        </a-form-item>
        <a-form-item label="绑定端口 (Bind Port)" name="bind_port" class="flex-1">
          <a-input-number v-model:value="formState.bind_port" :min="1" :max="65535" class="w-full" />
        </a-form-item>
      </div>

      <a-divider orientation="left" style="margin-top: 8px;">Agent 认证</a-divider>

      <a-form-item label="认证令牌 (Token)">
        <a-input v-model:value="authToken" placeholder="留空则使用全局配置" />
      </a-form-item>

      <a-form-item label="认证模式 (Auth Mode)">
        <a-radio-group v-model:value="authMode" button-style="solid">
          <a-radio-button value="plain_token">共享令牌</a-radio-button>
          <a-radio-button value="challenge_response">挑战-响应</a-radio-button>
        </a-radio-group>
      </a-form-item>
    </a-form>
  </a-modal>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue';
import type { FormInstance } from 'ant-design-vue';
import { message } from '@/utils/message';
import { spawnListener, SpawnListenerRequest } from '@/api/listener';

const props = defineProps<{
  visible: boolean;
}>();

const emit = defineEmits(['update:visible', 'success']);

const formRef = ref<FormInstance>();
const submitting = ref(false);
const authToken = ref('');
const authMode = ref('plain_token');

const formState = reactive<SpawnListenerRequest>({
  name: '',
  kind: 'tcp_json',
  bind_host: '0.0.0.0',
  bind_port: 1234,
});

const rules = {
  name: [{ required: true, message: '请输入监听器名称' }],
  kind: [{ required: true, message: '请选择协议' }],
  bind_host: [{ required: true, message: '请输入绑定地址' }],
  bind_port: [{ required: true, message: '请输入合法端口', type: 'number' }],
};

const handleCancel = () => {
  emit('update:visible', false);
  formRef.value?.resetFields();
  authToken.value = '';
  authMode.value = 'plain_token';
};

const handleSubmit = async () => {
  try {
    await formRef.value?.validate();
    submitting.value = true;

    const config: Record<string, unknown> = {};
    if (authToken.value) {
      config.agent_token = authToken.value;
      config.agent_auth_mode = authMode.value;
    }

    await spawnListener({ ...formState, config: Object.keys(config).length ? config : undefined });
    message.success('监听器创建成功');
    emit('success');
    handleCancel();
  } catch (err: any) {
    if (err.errorFields) return; // Validation failed natively
    message.error(err.message || '操作失败');
  } finally {
    submitting.value = false;
  }
};
</script>
