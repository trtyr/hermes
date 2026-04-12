import { useConnectionStore } from '@/store/connection';

export interface ListenerRecord {
  id: string;
  name: string;
  protocol: string;
  bind_host: string;
  bind_port: number;
  status: string;
  created_at: number;
}

export interface SpawnListenerRequest {
  name: string;
  protocol: string;
  bind_host: string;
  bind_port: number;
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

function getBaseUrl() {
  const store = useConnectionStore();
  const profile = store.activeProfile;
  if (!profile) throw new Error('未连接到后端服务器');
  return profile.server_url;
}

export async function fetchListeners(): Promise<{ success: boolean; listeners: ListenerRecord[] }> {
  const url = `${getBaseUrl()}/listeners`;
  const res = await fetch(url, { headers: getAuthHeaders() });
  if (!res.ok) throw new Error('获取监听器失败: ' + res.statusText);
  return res.json();
}

export async function spawnListener(data: SpawnListenerRequest): Promise<{ success: boolean; listener_id: string }> {
  const url = `${getBaseUrl()}/listeners`;
  const res = await fetch(url, {
    method: 'POST',
    headers: getAuthHeaders(),
    body: JSON.stringify(data)
  });
  if (!res.ok) throw new Error('创建监听器失败: ' + res.statusText);
  return res.json();
}

export async function startListener(id: string): Promise<{ success: boolean }> {
  const url = `${getBaseUrl()}/listeners/${id}/enable`;
  const res = await fetch(url, {
    method: 'POST',
    headers: getAuthHeaders()
  });
  if (!res.ok) throw new Error('启动监听器失败: ' + res.statusText);
  return res.json();
}

export async function stopListener(id: string): Promise<{ success: boolean }> {
  const url = `${getBaseUrl()}/listeners/${id}/disable`;
  const res = await fetch(url, {
    method: 'POST',
    headers: getAuthHeaders()
  });
  if (!res.ok) throw new Error('停止监听器失败: ' + res.statusText);
  return res.json();
}

export async function deleteListener(id: string): Promise<{ success: boolean }> {
  const url = `${getBaseUrl()}/listeners/${id}`;
  const res = await fetch(url, {
    method: 'DELETE',
    headers: getAuthHeaders()
  });
  if (!res.ok) throw new Error('删除监听器失败: ' + res.statusText);
  return res.json();
}
