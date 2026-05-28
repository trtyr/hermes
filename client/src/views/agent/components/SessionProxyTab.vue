<template>
  <div class="flex flex-col h-full p-4 overflow-auto">
    <!-- Start proxy button -->
    <div class="flex items-center justify-between mb-4">
      <span class="text-sm text-slate-600">SOCKS5 代理隧道</span>
      <a-button
        type="primary"
        size="small"
        :loading="proxyStarting"
        @click="handleStartProxy"
      >
        <template #icon><PlusOutlined /></template>
        新建代理
      </a-button>
    </div>

    <!-- Empty state -->
    <div v-if="proxies.length === 0" class="flex flex-col items-center justify-center flex-1 text-slate-400">
      <ApiOutlined style="font-size: 36px; opacity: 0.3" />
      <p class="mt-3 text-sm">暂无代理会话</p>
      <p class="text-xs text-slate-300 mt-1">点击"新建代理"开启 SOCKS5 隧道</p>
    </div>

    <!-- Proxy cards -->
    <div v-else class="space-y-3">
      <div
        v-for="p in proxies"
        :key="p.proxy_id"
        class="border border-gray-200 rounded-lg p-3 hover:border-blue-200 transition-colors"
      >
        <div class="flex items-center justify-between mb-2">
          <div class="flex items-center gap-2">
            <span class="inline-block w-2 h-2 rounded-full" :class="p.status === 'open' ? 'bg-green-500' : 'bg-gray-300'"></span>
            <span class="font-mono text-sm text-slate-700">{{ p.proxy_id }}</span>
            <a-tag :color="p.status === 'open' ? 'green' : 'default'" size="small" class="mr-0">
              {{ p.status === 'open' ? '运行中' : p.status }}
            </a-tag>
          </div>
          <a-button
            type="link"
            size="small"
            :loading="proxyStopping === p.proxy_id"
            @click="handleStopProxy(p.proxy_id)"
          >
            停止
          </a-button>
        </div>

        <div class="text-xs text-slate-500 space-y-1">
          <div class="flex items-center gap-2">
            <span class="text-slate-400">绑定地址:</span>
            <code class="bg-gray-100 px-1.5 py-0.5 rounded font-mono text-blue-600">{{ p.bind_addr }}</code>
            <a-button
              type="link"
              size="small"
              class="!p-0 !h-auto !text-xs"
              @click="copyProxyAddr(p.bind_addr)"
            >
              <template #icon><CopyOutlined /></template>
            </a-button>
          </div>
          <div>
            <span class="text-slate-400">活跃流:</span>
            <span class="ml-1">{{ p.active_streams }}</span>
          </div>
          <div v-if="p.last_error" class="text-red-500">
            <span class="text-slate-400">错误:</span>
            <span class="ml-1">{{ p.last_error }}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Closed proxies section (reference only) -->
    <div v-if="closedProxies.length > 0" class="mt-4">
      <div class="text-xs text-slate-400 mb-2">已停止的代理 ({{ closedProxies.length }})</div>
      <div
        v-for="p in closedProxies"
        :key="'closed-' + p.proxy_id"
        class="border border-gray-100 rounded p-2 mb-2 opacity-60"
      >
        <div class="flex items-center justify-between">
          <div>
            <span class="font-mono text-xs text-slate-500">{{ p.proxy_id }}</span>
            <span class="text-xs text-slate-400 ml-2">{{ p.bind_addr }}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Help text -->
    <div v-if="proxies.length > 0" class="mt-3 p-2 bg-gray-50 rounded text-xs text-slate-500">
      <p class="font-medium mb-1">使用方法:</p>
      <code class="block">curl --socks5 {{ proxies[0]?.bind_addr || '127.0.0.1:PORT' }} http://内网地址</code>
      <p class="mt-1 text-slate-400">支持标准 SOCKS5 协议的客户端均可使用</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { message } from 'ant-design-vue';
import { ApiOutlined, CopyOutlined, PlusOutlined } from '@ant-design/icons-vue';
import { listProxies, startProxy as apiStartProxy, deleteProxy as apiDeleteProxy } from '@/api/proxy';
import type { ProxySessionRecord } from '@/api/proxy';

const props = defineProps<{ agentId: string }>();

const proxies = ref<ProxySessionRecord[]>([]);
const closedProxies = ref<ProxySessionRecord[]>([]);
const proxyStarting = ref(false);
const proxyStopping = ref<string | null>(null);

async function loadProxies() {
  try {
    const res = await listProxies(props.agentId);
    const all = res.proxies || [];
    proxies.value = all.filter(p => p.status === 'open');
    closedProxies.value = all.filter(p => p.status !== 'open');
  } catch {
    // silently fail
  }
}

const handleStartProxy = async () => {
  proxyStarting.value = true;
  try {
    const res = await apiStartProxy(props.agentId);
    message.success(`代理已启动: ${res.proxy.bind_addr}`);
    await loadProxies();
  } catch (e: any) {
    message.error(e.message || '启动代理失败');
  } finally {
    proxyStarting.value = false;
  }
};

const handleStopProxy = async (proxyId: string) => {
  proxyStopping.value = proxyId;
  try {
    await apiDeleteProxy(props.agentId, proxyId);
    message.success(`代理 ${proxyId} 已删除`);
    await loadProxies();
  } catch (e: any) {
    message.error(e.message || '停止代理失败');
  } finally {
    proxyStopping.value = null;
  }
};

const copyProxyAddr = (addr: string) => {
  navigator.clipboard.writeText(addr).then(() => {
    message.success(`已复制: ${addr}`);
  });
};

defineExpose({ loadProxies });
</script>
