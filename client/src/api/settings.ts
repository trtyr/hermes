import { useConnectionStore } from '@/store/connection';

export interface AgentAuthSettings {
  agent_token: string;
  agent_auth_mode: string;
}

function getBaseUrl() {
  const store = useConnectionStore();
  const profile = store.activeProfile;
  if (!profile) throw new Error('未连接');
  return profile.server_url;
}

function getAuthHeaders() {
  const store = useConnectionStore();
  const profile = store.activeProfile;
  if (!profile) throw new Error('未连接');
  return { 'Authorization': `Bearer ${profile.api_token}`, 'Content-Type': 'application/json' };
}

export async function getAuthSettings(): Promise<AgentAuthSettings> {
  const res = await fetch(`${getBaseUrl()}/server/auth-settings`, { headers: getAuthHeaders() });
  if (!res.ok) throw new Error('获取认证设置失败');
  return res.json();
}

export async function updateAuthSettings(settings: AgentAuthSettings): Promise<{ success: boolean; detail: string }> {
  const res = await fetch(`${getBaseUrl()}/server/auth-settings`, {
    method: 'PUT',
    headers: getAuthHeaders(),
    body: JSON.stringify(settings)
  });
  if (!res.ok) throw new Error('更新认证设置失败');
  return res.json();
}
