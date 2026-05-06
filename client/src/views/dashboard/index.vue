<template>
  <div class="h-full w-full p-4 relative">
    <!-- Connected State (always — route guard ensures valid session) -->
    <div class="h-full flex flex-col">
      <!-- Header with Status -->
      <div class="flex justify-between items-center mb-6">
        <h2 class="text-2xl font-semibold text-slate-800">控制台总览</h2>
        <ConnectionBadge />
      </div>

      <!-- Loading / Error Spinners -->
      <div v-if="loading" class="flex-1 flex flex-col items-center justify-center">
        <a-spin size="large" />
        <div class="mt-4 text-slate-400">正在加载统计数据...</div>
      </div>
      
      <div v-else-if="error" class="flex-1 flex flex-col items-center justify-center text-center">
        <div class="w-16 h-16 bg-red-50 text-red-500 rounded-full flex items-center justify-center mb-4">
          <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"></path>
          </svg>
        </div>
        <h2 class="text-xl font-semibold text-slate-800 mb-2">获取统计数据失败</h2>
        <p class="text-slate-500 mb-4 max-w-md">{{ error }}</p>
        <a-button @click="loadStats">重试</a-button>
      </div>

      <!-- Data Dashboard -->
      <div v-else-if="stats" class="space-y-6">
        <TopStatsGrid :stats="stats" />
        
        <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
          <ServerInfoCard :stats="stats" />
          <AgentsDistCard :stats="stats" />
          <ListenersDistCard :stats="stats" />
        </div>

        <!-- Agent认证设置 -->
        <a-card title="Agent 认证设置" class="dashboard-card" :bordered="false">
          <template #extra>
            <a-tooltip title="配置 Agent 连接时的认证方式和令牌。修改后约1秒生效，无需重启。">
              <QuestionCircleOutlined class="text-slate-400" />
            </a-tooltip>
          </template>
          <a-form layout="vertical" :model="authForm">
            <a-form-item label="认证令牌 (Agent Token)">
              <a-input-password
                v-model:value="authForm.agent_token"
                placeholder="留空则不启用认证"
                :disabled="authLoading"
              />
              <div class="text-xs text-slate-400 mt-1">
                此令牌需在生成 Agent 时一并嵌入
              </div>
            </a-form-item>
            <a-form-item label="认证模式">
              <a-radio-group v-model:value="authForm.agent_auth_mode" :disabled="authLoading">
                <a-radio-button value="plain_token">
                  <template #default>
                    <span class="flex items-center gap-1">
                      <UnlockOutlined />
                      共享令牌
                    </span>
                  </template>
                </a-radio-button>
                <a-radio-button value="challenge_response">
                  <template #default>
                    <span class="flex items-center gap-1">
                      <SafetyCertificateOutlined />
                      挑战-响应 (HMAC)
                    </span>
                  </template>
                </a-radio-button>
              </a-radio-group>
              <div class="text-xs text-slate-400 mt-1">
                <template v-if="authForm.agent_auth_mode === 'plain_token'">
                  Agent 注册时明文携带令牌，Server 对比验证
                </template>
                <template v-else>
                  Server 发送随机 nonce，Agent 用 HMAC-SHA256 签名响应，令牌不传输
                </template>
              </div>
            </a-form-item>
            <a-form-item>
              <a-button type="primary" :loading="authLoading" @click="saveAuthSettings" :disabled="!authForm.agent_auth_mode">
                <template #icon><SaveOutlined /></template>
                保存设置
              </a-button>
              <a-button class="ml-2" :loading="authLoading" @click="loadAuthSettings">
                刷新
              </a-button>
            </a-form-item>
          </a-form>
        </a-card>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, onUnmounted, watch } from 'vue';
import { message } from 'ant-design-vue';
import { QuestionCircleOutlined, UnlockOutlined, SafetyCertificateOutlined, SaveOutlined } from '@ant-design/icons-vue';

// Subcomponents
import ConnectionBadge from './components/ConnectionBadge.vue';
import TopStatsGrid from './components/TopStatsGrid.vue';
import ServerInfoCard from './components/ServerInfoCard.vue';
import AgentsDistCard from './components/AgentsDistCard.vue';
import ListenersDistCard from './components/ListenersDistCard.vue';

// State and Networking
import { useConnectionStore } from '@/store/connection';
import { useEventStore } from '@/store/events';
import { fetchDashboardStats } from '@/api/dashboard';
import type { DashboardStats } from '@/api/dashboard';
import { getAuthSettings, updateAuthSettings } from '@/api/settings';

const connectionStore = useConnectionStore();
const eventStore = useEventStore();

const stats = ref<DashboardStats | null>(null);
const loading = ref(false);
const error = ref('');

async function loadStats() {
  if (!connectionStore.activeProfile) return;
  
  loading.value = true;
  error.value = '';
  
  try {
    stats.value = await fetchDashboardStats();
  } catch (err: any) {
    console.error('Failed to load dashboard stats:', err);
    error.value = err.message || '网络请求失败，请检查后端是否正常运行。';
  } finally {
    loading.value = false;
  }
}

watch(() => connectionStore.activeProfileId, (newId) => {
  if (newId) {
    loadStats();
  } else {
    stats.value = null;
  }
});

// Agent Auth Settings
const authForm = reactive({ agent_token: '', agent_auth_mode: 'plain_token' });
const authLoading = ref(false);

const loadAuthSettings = async () => {
  try {
    const settings = await getAuthSettings();
    authForm.agent_token = settings.agent_token || '';
    authForm.agent_auth_mode = settings.agent_auth_mode || 'plain_token';
  } catch (_e: any) {
    // silently fail - user can click refresh
  }
};

const saveAuthSettings = async () => {
  authLoading.value = true;
  try {
    const result = await updateAuthSettings({
      agent_token: authForm.agent_token,
      agent_auth_mode: authForm.agent_auth_mode
    });
    message.success(result.detail || '认证设置已更新');
  } catch (e: any) {
    message.error(e.message || '保存失败');
  } finally {
    authLoading.value = false;
  }
};

// Auto-refresh: debounced event-driven + periodic fallback
let debounceTimer: ReturnType<typeof setTimeout> | null = null;
let periodicTimer: ReturnType<typeof setInterval> | null = null;
let unsubscribe: (() => void) | null = null;

function scheduleDebouncedReload() {
  if (debounceTimer) clearTimeout(debounceTimer);
  debounceTimer = setTimeout(() => {
    loadStats();
  }, 3000);
}

onMounted(() => {
  if (connectionStore.activeProfile) {
    loadStats();
    loadAuthSettings();
  }

  // WebSocket-driven refresh with 3s debounce
  unsubscribe = eventStore.subscribe(() => {
    scheduleDebouncedReload();
  });

  // Periodic fallback every 30s
  periodicTimer = setInterval(() => {
    if (connectionStore.activeProfile) {
      loadStats();
    }
  }, 30_000);
});

onUnmounted(() => {
  if (debounceTimer) clearTimeout(debounceTimer);
  if (periodicTimer) clearInterval(periodicTimer);
  if (unsubscribe) unsubscribe();
});
</script>
