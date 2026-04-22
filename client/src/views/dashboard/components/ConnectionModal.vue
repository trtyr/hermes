<template>
  <a-modal
    :open="visible"
    title="后端连接管理"
    :footer="null"
    width="600px"
    destroyOnClose
    @update:open="$emit('update:visible', $event)"
  >
    <div class="mt-4">
      <!-- Profile List -->
      <div class="mb-6">
        <div class="flex justify-between items-center mb-2">
          <h3 class="text-sm font-medium text-slate-700">已保存的连接</h3>
          <a-button type="link" size="small" @click="startNewConnection">添加新连接</a-button>
        </div>
        
        <div v-if="connectionStore.profiles.length === 0" class="text-center py-4 text-slate-400 bg-slate-50 rounded-md border border-dashed border-slate-200">
          暂无保存的连接配置
        </div>
        
        <div v-else class="space-y-2 max-h-48 overflow-y-auto pr-1">
          <div 
            v-for="profile in connectionStore.profiles" 
            :key="profile.id"
            class="flex items-center justify-between p-3 rounded-md border transition-colors cursor-pointer"
            :class="connectionStore.activeProfileId === profile.id ? 'border-blue-500 bg-blue-50' : 'border-slate-200 bg-white hover:border-blue-300'"
            @click="selectAndConnect(profile.id)"
          >
            <div class="flex-1 min-w-0">
              <div class="flex items-center space-x-2">
                <span class="font-medium text-slate-800 truncate">{{ profile.connection_name || '未命名连接' }}</span>
                <span v-if="connectionStore.activeProfileId === profile.id" class="text-[10px] px-1.5 py-0.5 bg-blue-100 text-blue-600 rounded text-nowrap">当前使用</span>
              </div>
              <div class="text-xs text-slate-500 truncate mt-1">{{ profile.server_url }}</div>
            </div>
            <div class="flex space-x-1 ml-4" @click.stop="() => {}">
              <a-button type="text" size="small" @click.stop="editProfile(profile)"><EditOutlined /></a-button>
              <a-popconfirm
                title="确定要删除这个连接配置吗？"
                @confirm="connectionStore.deleteProfile(profile.id)"
              >
                <a-button type="text" danger size="small" @click.stop="() => {}"><DeleteOutlined /></a-button>
              </a-popconfirm>
            </div>
          </div>
        </div>
      </div>

      <a-divider v-if="isEditing || connectionStore.profiles.length === 0" />

      <!-- Edit/Add Form -->
      <div v-if="isEditing || connectionStore.profiles.length === 0">
        <h3 class="text-sm font-medium text-slate-700 mb-4">{{ editingId ? '编辑连接' : '新建后端连接' }}</h3>
        
        <a-form :model="formState" layout="vertical">
          <a-form-item label="连接名称 (可选)">
            <a-input v-model:value="formState.connection_name" placeholder="例如: Local Dev, Teamserver" />
          </a-form-item>
          
          <a-form-item label="服务端地址" required>
            <a-input v-model:value="formState.server_url" placeholder="例如: http://127.0.0.1:3000" />
          </a-form-item>
          
          <a-form-item label="API Token" required>
            <a-input-password v-model:value="formState.api_token" placeholder="输入服务端的 config.toml 中的 api_token" />
          </a-form-item>
          
          <div class="bg-slate-50 p-3 rounded-md mb-4 text-xs text-slate-500">
            <div class="mb-1 text-slate-700 flex items-center space-x-1.5 font-medium">
              <InfoCircleOutlined /> <span>测试状态</span>
            </div>
            <div class="flex flex-col space-y-1 mt-2">
              <div class="flex items-center space-x-2">
                <span class="w-2 h-2 rounded-full" :class="testStatus.reachability === 'success' ? 'bg-green-500' : testStatus.reachability === 'error' ? 'bg-red-500' : 'bg-slate-300'"></span>
                <span>连通性检查 (GET /health)</span>
              </div>
              <div class="flex items-center space-x-2">
                <span class="w-2 h-2 rounded-full" :class="testStatus.auth === 'success' ? 'bg-green-500' : testStatus.auth === 'error' ? 'bg-red-500' : 'bg-slate-300'"></span>
                <span>Token 验证 (GET /tasks)</span>
              </div>
            </div>
            <div v-if="testMessage" class="mt-2 text-red-500">{{ testMessage }}</div>
          </div>

          <div class="flex justify-end gap-2">
            <a-button @click="cancelEdit" v-if="connectionStore.profiles.length > 0">取消</a-button>
            <a-button @click="runTest" :loading="isTesting">测试连接</a-button>
            <a-button type="primary" @click="saveConnection" :disabled="!formState.server_url || !formState.api_token">保存并使用</a-button>
          </div>
        </a-form>
      </div>
    </div>
  </a-modal>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue';
import { message } from 'ant-design-vue';
import { DeleteOutlined, EditOutlined, InfoCircleOutlined } from '@ant-design/icons-vue';
import { useConnectionStore } from '@/store/connection';
import type { BackendProfile } from '@/store/connection';
import { testConnection } from '@/api/connection';

const props = defineProps<{ visible: boolean }>();
const emit = defineEmits(['update:visible']);

const connectionStore = useConnectionStore();

const isEditing = ref(false);
const editingId = ref<string | null>(null);
const isTesting = ref(false);
const testMessage = ref('');
const testStatus = reactive({
  reachability: 'pending' as 'pending' | 'success' | 'error',
  auth: 'pending' as 'pending' | 'success' | 'error'
});

const formState = reactive({
  connection_name: '',
  server_url: '',
  api_token: ''
});

function resetForm() {
  formState.connection_name = '';
  formState.server_url = '';
  formState.api_token = '';
  editingId.value = null;
  testMessage.value = '';
  testStatus.reachability = 'pending';
  testStatus.auth = 'pending';
}

function startNewConnection() {
  resetForm();
  isEditing.value = true;
}

function editProfile(profile: BackendProfile) {
  formState.connection_name = profile.connection_name;
  formState.server_url = profile.server_url;
  formState.api_token = profile.api_token;
  editingId.value = profile.id;
  isEditing.value = true;
  testMessage.value = '';
  testStatus.reachability = 'pending';
  testStatus.auth = 'pending';
}

function cancelEdit() {
  isEditing.value = false;
  resetForm();
}

async function runTest() {
  if (!formState.server_url || !formState.api_token) {
    message.warning('请先填写服务端地址和 API Token');
    return false;
  }

  isTesting.value = true;
  testMessage.value = '';
  testStatus.reachability = 'pending';
  testStatus.auth = 'pending';

  try {
    const result = await testConnection(formState.server_url, formState.api_token);
    
    if (result.success) {
      testStatus.reachability = 'success';
      testStatus.auth = 'success';
      message.success('连接测试成功！');
      return true;
    } else {
      if (result.errorType === 'network') {
        testStatus.reachability = 'error';
        testStatus.auth = 'pending';
      } else {
        testStatus.reachability = 'success';
        testStatus.auth = 'error';
      }
      testMessage.value = result.message || '连接失败';
      return false;
    }
  } catch (error: any) {
    testStatus.reachability = 'error';
    testStatus.auth = 'pending';
    testMessage.value = error.message || '网络请求异常';
    return false;
  } finally {
    isTesting.value = false;
  }
}

function selectAndConnect(id: string) {
  connectionStore.setActiveProfile(id);
  emit('update:visible', false);
}

async function saveConnection() {
  if (!formState.server_url || !formState.api_token) {
    message.warning('请先填写服务端地址和 API Token');
    return;
  }

  formState.server_url = connectionStore.normalizeUrl(formState.server_url);

  const testPassed = await runTest();
  if (!testPassed) return;

  if (editingId.value) {
    connectionStore.updateProfile(editingId.value, {
      connection_name: formState.connection_name,
      server_url: formState.server_url,
      api_token: formState.api_token
    });
    connectionStore.setActiveProfile(editingId.value);
  } else {
    const newProfile = connectionStore.addProfile({
      connection_name: formState.connection_name,
      server_url: formState.server_url,
      api_token: formState.api_token
    });
    connectionStore.setActiveProfile(newProfile.id);
  }

  message.success('已保存并连接到后端！');
  isEditing.value = false;
  emit('update:visible', false);
}
</script>
