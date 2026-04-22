<template>
  <div class="relative w-full h-screen overflow-hidden bg-white flex justify-center">
    <!-- Main card -->
    <div class="flex w-full h-full max-w-[1200px] shadow-2xl overflow-hidden rounded-none md:rounded-2xl md:h-[620px] md:my-auto md:w-4/5 lg:w-[1000px] bg-white">
      <!-- Left: Branding -->
      <div class="hidden md:flex flex-col justify-center items-center w-1/2 bg-blue-600 relative overflow-hidden text-white p-10">
        <!-- Blob decorations -->
        <div class="absolute top-[-10%] left-[-10%] w-64 h-64 bg-blue-500 rounded-full mix-blend-multiply filter blur-2xl opacity-70 animate-blob"></div>
        <div class="absolute bottom-[-10%] right-[-10%] w-64 h-64 bg-indigo-500 rounded-full mix-blend-multiply filter blur-2xl opacity-70 animate-blob animation-delay-2000"></div>

        <div class="relative z-10 w-full">
          <div class="flex items-center space-x-3 mb-6">
            <svg class="w-12 h-12 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
            </svg>
            <span class="text-4xl font-bold tracking-wider">Hermes C2</span>
          </div>
          <div class="text-lg text-blue-100 mt-4 leading-relaxed max-w-sm">
            下一代命令与控制基础设施。<br/>用于高级安全操作的模块化框架。
          </div>
        </div>

        <div class="relative z-10 mt-12 w-full flex justify-center">
          <img :src="loginBoxBg" class="w-4/5 object-contain opacity-90" alt="Login Illustration" />
        </div>
      </div>

      <!-- Right: Login Form -->
      <div class="w-full md:w-1/2 flex flex-col justify-center p-8 lg:p-14 bg-white transition-colors duration-300">
        <div class="w-full max-w-md mx-auto">
          <!-- Mobile Branding -->
          <div class="flex items-center space-x-3 mb-8 md:hidden justify-center text-gray-900">
            <svg class="w-10 h-10 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
            </svg>
            <span class="text-3xl font-bold">Hermes C2</span>
          </div>

          <h2 class="text-2xl font-semibold text-gray-800 mb-2">连接到服务器</h2>
          <p class="text-sm text-gray-500 mb-8">输入服务器地址和凭据以访问控制台</p>

          <a-form :model="formState" @finish="handleFinish" layout="vertical" class="w-full">
            <!-- Server URL -->
            <a-form-item label="服务器地址" name="serverUrl" :rules="[{ required: true, message: '请输入服务器地址' }]">
              <a-input
                v-model:value="formState.serverUrl"
                size="large"
                placeholder="127.0.0.1:3000"
              >
                <template #prefix>
                  <GlobalOutlined class="text-gray-400" />
                </template>
              </a-input>
            </a-form-item>

            <!-- Username -->
            <a-form-item label="用户名" name="username" :rules="[{ required: true, message: '请输入用户名' }]">
              <a-input
                v-model:value="formState.username"
                size="large"
                placeholder="admin"
              >
                <template #prefix>
                  <UserOutlined class="text-gray-400" />
                </template>
              </a-input>
            </a-form-item>

            <!-- Password -->
            <a-form-item label="密码" name="password" :rules="[{ required: true, message: '请输入密码' }]">
              <a-input-password
                v-model:value="formState.password"
                size="large"
                placeholder="••••••"
                @pressEnter="handleFinish"
              >
                <template #prefix>
                  <LockOutlined class="text-gray-400" />
                </template>
              </a-input-password>
            </a-form-item>

            <!-- Error message -->
            <div v-if="errorMsg" class="text-red-500 text-sm mb-4 flex items-center">
              <ExclamationCircleOutlined class="mr-1.5" />
              {{ errorMsg }}
            </div>

            <!-- Submit -->
            <a-form-item class="mb-0 mt-6">
              <a-button
                type="primary"
                html-type="submit"
                size="large"
                class="w-full h-12 text-lg tracking-widest"
                :loading="loading"
              >
                连接
              </a-button>
            </a-form-item>
          </a-form>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import loginBoxBg from '@/assets/login-box-bg.svg';
import { reactive, ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import {
  UserOutlined,
  LockOutlined,
  GlobalOutlined,
  ExclamationCircleOutlined,
} from '@ant-design/icons-vue';
import { useConnectionStore } from '@/store/connection';
import { loginToBackend } from '@/api/connection';

const router = useRouter();
const loading = ref(false);
const errorMsg = ref('');
const connectionStore = useConnectionStore();

const formState = reactive({
  serverUrl: '',
  username: '',
  password: '',
});

onMounted(() => {
  // Pre-fill from last active profile
  if (connectionStore.activeProfile) {
    formState.serverUrl = connectionStore.activeProfile.server_url;
  }
});

const handleFinish = async () => {
  errorMsg.value = '';
  loading.value = true;

  try {
    const result = await loginToBackend(
      formState.serverUrl,
      formState.username,
      formState.password
    );

    if (!result.success) {
      errorMsg.value = result.error || '连接失败';
      loading.value = false;
      return;
    }

    // Save or update connection profile with session token
    const normalizedUrl = connectionStore.normalizeUrl(formState.serverUrl);
    const existing = connectionStore.profiles.find(
      (p) => p.server_url === normalizedUrl
    );

    if (existing) {
      connectionStore.updateProfile(existing.id, {
        api_token: result.session_token || '',
        connection_name: formState.username,
      });
      connectionStore.setActiveProfile(existing.id);
    } else {
      const profile = connectionStore.addProfile({
        connection_name: formState.username,
        server_url: normalizedUrl,
        api_token: result.session_token || '',
      });
      connectionStore.setActiveProfile(profile.id);
    }

    router.push('/dashboard');
  } catch {
    errorMsg.value = '连接异常，请检查服务器地址';
  } finally {
    loading.value = false;
  }
};
</script>

<style scoped>
@reference "tailwindcss";

@keyframes blob {
  0% { transform: translate(0px, 0px) scale(1); }
  33% { transform: translate(30px, -50px) scale(1.1); }
  66% { transform: translate(-20px, 20px) scale(0.9); }
  100% { transform: translate(0px, 0px) scale(1); }
}
.animate-blob {
  animation: blob 7s infinite;
}
.animation-delay-2000 {
  animation-delay: 2s;
}
</style>
