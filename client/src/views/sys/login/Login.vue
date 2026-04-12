<template>
  <div class="relative w-full h-screen overflow-hidden bg-white dark:bg-[#14161A] flex justify-center">
    <!-- Header / Nav area -->
    <div class="absolute top-4 right-4 flex items-center space-x-4 z-50">
      <div class="cursor-pointer text-gray-500 hover:text-gray-900 dark:text-gray-400 dark:hover:text-white transition-colors" @click="toggleDark">
        <svg v-if="!appStore.isDark" class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"></path>
        </svg>
        <svg v-else class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"></path>
        </svg>
      </div>
    </div>

    <div class="flex w-full h-full max-w-[1200px] shadow-2xl overflow-hidden rounded-none md:rounded-2xl md:h-[600px] md:my-auto md:w-4/5 lg:w-[1000px] bg-white dark:bg-[#1C1E22]">
      <!-- Left Box (Branding) -->
      <div class="hidden md:flex flex-col justify-center items-center w-1/2 bg-blue-600 relative overflow-hidden text-white p-10">
        <!-- Abstract decorations -->
        <div class="absolute top-[-10%] left-[-10%] w-64 h-64 bg-blue-500 rounded-full mix-blend-multiply filter blur-2xl opacity-70 animate-blob"></div>
        <div class="absolute bottom-[-10%] right-[-10%] w-64 h-64 bg-indigo-500 rounded-full mix-blend-multiply filter blur-2xl opacity-70 animate-blob animation-delay-2000"></div>
        
        <div class="relative z-10 w-full">
          <div class="flex items-center space-x-3 mb-6">
            <svg class="w-12 h-12 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path>
            </svg>
            <span class="text-4xl font-bold tracking-wider">Hermes C2</span>
          </div>
          <div class="text-lg text-blue-100 mt-4 leading-relaxed max-w-sm">
            下一代命令与控制基础设施。<br/>用于高级安全操作的模块化框架。
          </div>
        </div>
        
        <!-- Illustration Placeholder -->
        <div class="relative z-10 mt-12 w-full flex justify-center">
          <img :src="loginBoxBg" class="w-4/5 object-contain opacity-90" alt="Login Illustration" />
        </div>
      </div>

      <!-- Right Box (Login Form) -->
      <div class="w-full md:w-1/2 flex flex-col justify-center p-8 lg:p-14 bg-white dark:bg-[#1C1E22] transition-colors duration-300">
        <div class="w-full max-w-md mx-auto">
          <!-- Mobile Branding -->
          <div class="flex items-center space-x-3 mb-8 md:hidden justify-center text-gray-900 dark:text-white">
            <svg class="w-10 h-10 text-blue-600 dark:text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path>
            </svg>
            <span class="text-3xl font-bold">Hermes C2</span>
          </div>

          <h2 class="text-2xl font-semibold text-gray-800 dark:text-gray-100 mb-2">登录</h2>
          <p class="text-sm text-gray-500 dark:text-gray-400 mb-8">请输入您的凭据以访问控制台</p>

          <a-form :model="formState" :rules="rules" @finish="handleFinish" layout="vertical" class="w-full">
            <a-form-item name="username">
              <a-input v-model:value="formState.username" size="large" placeholder="用户名" class="py-2">
                <template #prefix>
                  <UserOutlined class="text-gray-400" />
                </template>
              </a-input>
            </a-form-item>

            <a-form-item name="password" class="mb-8">
              <a-input-password v-model:value="formState.password" size="large" placeholder="密码" class="py-2">
                <template #prefix>
                  <LockOutlined class="text-gray-400" />
                </template>
              </a-input-password>
            </a-form-item>

            <a-form-item>
              <a-button type="primary" html-type="submit" size="large" class="w-full h-12 text-lg tracking-widest" :loading="loading">
                登录
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
  LockOutlined
} from '@ant-design/icons-vue';
import { message } from 'ant-design-vue';
import { useAppStore } from '@/store/app';
import { parse } from 'smol-toml';

const router = useRouter();
const loading = ref(false);
const appStore = useAppStore();

const formState = reactive({
  username: '',
  password: '',
});

const rules = {
  username: [{ required: true, message: '请输入用户名！', trigger: 'blur' }],
  password: [{ required: true, message: '请输入密码！', trigger: 'blur' }],
};

const toggleDark = () => {
  appStore.toggleTheme();
};

onMounted(() => {
  // Check system preference but don't force toggle if it conflicts with store
  if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
    if (!appStore.isDark) {
      appStore.toggleTheme();
    }
  }
});

// 使用原生的 Web Crypto API 生成密码的 SHA-256 哈希
const hashPassword = async (password: string) => {
  const msgBuffer = new TextEncoder().encode(password);
  const hashBuffer = await crypto.subtle.digest('SHA-256', msgBuffer);
  const hashArray = Array.from(new Uint8Array(hashBuffer));
  return hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
};

const handleFinish = async (values: any) => {
  loading.value = true;
  
  try {
    // 读取位于 public/ 目录下的 config.toml
    const response = await fetch('/config.toml');
    if (!response.ok) {
      throw new Error('网络请求失败：无法加载配置文件');
    }
    
    const tomlText = await response.text();
    const config = parse(tomlText) as any;
    
    // 对用户输入的密码进行 Hash
    const inputHash = await hashPassword(values.password);

    // 验证账号与密码 Hash
    setTimeout(() => {
      loading.value = false;
      if (
        config.auth &&
        values.username === config.auth.username && 
        inputHash === config.auth.password_hash
      ) {
        message.success('登录成功！');
        router.push('/dashboard');
      } else {
        message.error('用户名或密码错误！');
      }
    }, 800); // 模拟网络延迟
    
  } catch (error) {
    console.error('配置读取失败:', error);
    loading.value = false;
    message.error('无法读取本地配置文件！');
  }
};
</script>

<style scoped>
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
