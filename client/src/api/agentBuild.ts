import { useConnectionStore } from '@/store/connection';

export interface AgentBuildRecord {
  build_id: number;
  target_triple: string;
  profile: string;
  listener_id: number | null;
  server_addr: string;
  embedded_agent_token: boolean;
  artifact_path: string | null;
  artifact_name: string | null;
  status: 'pending' | 'succeeded' | 'failed';
  detail: string | null;
  created_at: number;
  updated_at: number;
}

export interface AgentBuildListResponse {
  builds: AgentBuildRecord[];
  total: number;
  limit: number;
  offset: number;
}

export interface CreateAgentBuildRequest {
  target_triple?: string;
  listener_id?: number;
  server_addr?: string;
  agent_token?: string;
  profile?: string;
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

export async function fetchAgentBuilds(params: { status?: string; target_triple?: string; limit?: number; offset?: number } = {}): Promise<AgentBuildListResponse> {
  const url = new URL(`${getBaseUrl()}/agent-builds`);
  if (params.status) url.searchParams.append('status', params.status);
  if (params.target_triple) url.searchParams.append('target_triple', params.target_triple);
  if (params.limit !== undefined) url.searchParams.append('limit', params.limit.toString());
  if (params.offset !== undefined) url.searchParams.append('offset', params.offset.toString());

  const res = await fetch(url.toString(), { headers: getAuthHeaders() });
  if (!res.ok) throw new Error('获取构建列表失败: ' + res.statusText);
  return res.json();
}

export async function createAgentBuild(data: CreateAgentBuildRequest): Promise<{ success: boolean; build: AgentBuildRecord }> {
  const res = await fetch(`${getBaseUrl()}/agent-builds`, {
    method: 'POST',
    headers: getAuthHeaders(),
    body: JSON.stringify(data)
  });
  if (!res.ok) throw new Error('创建构建失败: ' + res.statusText);
  return res.json();
}

export async function fetchAgentBuild(buildId: number): Promise<AgentBuildRecord> {
  const res = await fetch(`${getBaseUrl()}/agent-builds/${buildId}`, { headers: getAuthHeaders() });
  if (!res.ok) throw new Error('获取构建详情失败: ' + res.statusText);
  return res.json();
}

export function getBuildDownloadUrl(buildId: number): string {
  return `${getBaseUrl()}/agent-builds/${buildId}/download`;
}

export async function deleteAgentBuild(buildId: number): Promise<{ success: boolean }> {
  const res = await fetch(`${getBaseUrl()}/agent-builds/${buildId}`, {
    method: 'DELETE',
    headers: getAuthHeaders(),
  });
  if (!res.ok) {
    const data = await res.json().catch(() => null);
    throw new Error(data?.detail || '删除构建失败: ' + res.statusText);
  }
  return res.json();
}


