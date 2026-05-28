import { apiFetch } from './request';

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

export async function listProxies(agentId: string): Promise<ProxyListResponse> {
  return apiFetch<ProxyListResponse>(`/agents/${agentId}/proxy`);
}

export async function startProxy(agentId: string): Promise<ProxyStartResponse> {
  return apiFetch<ProxyStartResponse>(`/agents/${agentId}/proxy`, { method: 'POST' });
}

export async function deleteProxy(agentId: string, proxyId: string): Promise<ProxyStartResponse> {
  return apiFetch<ProxyStartResponse>(`/agents/${agentId}/proxy/${proxyId}`, { method: 'DELETE' });
}
