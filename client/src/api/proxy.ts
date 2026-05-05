import { useConnectionStore } from '@/store/connection';

export interface ProxySessionRecord {
  proxy_id: string;
  agent_id: string;
  bind_addr: string;
  status: string;
  active_streams: number;
  created_at: number;
  updated_at: number;
  last_error: string | null;
}

export interface ProxyListResponse {
  proxies: ProxySessionRecord[];
}

export interface ProxyStartResponse {
  success: boolean;
  detail: string;
  proxy: ProxySessionRecord;
}

function getBaseUrl() {
  const store = useConnectionStore();
  const profile = store.activeProfile;
  if (!profile) throw new Error('未连接到后端服务器');
  return profile.server_url;
}

function getAuthHeaders() {
  const store = useConnectionStore();
  const profile = store.activeProfile;
  if (!profile) throw new Error('未连接到后端服务器');
  return {
    'Authorization': `Bearer ${profile.api_token}`,
    'Content-Type': 'application/json'
  };
}

export async function listProxies(agentId: string): Promise<ProxyListResponse> {
  const baseUrl = getBaseUrl();
  const res = await fetch(`${baseUrl}/agents/${agentId}/proxy`, {
    headers: getAuthHeaders()
  });
  if (!res.ok) throw new Error(`获取代理列表失败: ${res.statusText}`);
  return res.json();
}

export async function startProxy(agentId: string): Promise<ProxyStartResponse> {
  const baseUrl = getBaseUrl();
  const res = await fetch(`${baseUrl}/agents/${agentId}/proxy`, {
    method: 'POST',
    headers: getAuthHeaders()
  });
  if (!res.ok) throw new Error(`启动代理失败: ${res.statusText}`);
  return res.json();
}

export async function deleteProxy(agentId: string, proxyId: string): Promise<ProxyStartResponse> {
  const baseUrl = getBaseUrl();
  const res = await fetch(`${baseUrl}/agents/${agentId}/proxy/${proxyId}`, {
    method: 'DELETE',
    headers: getAuthHeaders()
  });
  if (!res.ok) throw new Error(`删除代理失败: ${res.statusText}`);
  return res.json();
}
