import { apiFetch } from './request';

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

export async function fetchAgentBuilds(params: { status?: string; target_triple?: string; limit?: number; offset?: number } = {}): Promise<AgentBuildListResponse> {
  const searchParams = new URLSearchParams();
  if (params.status) searchParams.append('status', params.status);
  if (params.target_triple) searchParams.append('target_triple', params.target_triple);
  if (params.limit !== undefined) searchParams.append('limit', params.limit.toString());
  if (params.offset !== undefined) searchParams.append('offset', params.offset.toString());

  const qs = searchParams.toString();
  return apiFetch<AgentBuildListResponse>(`/agent-builds${qs ? `?${qs}` : ''}`);
}

export async function createAgentBuild(data: CreateAgentBuildRequest): Promise<{ success: boolean; build: AgentBuildRecord }> {
  return apiFetch<{ success: boolean; build: AgentBuildRecord }>('/agent-builds', {
    method: 'POST',
    body: JSON.stringify(data),
  });
}

export async function fetchAgentBuild(buildId: number): Promise<AgentBuildRecord> {
  return apiFetch<AgentBuildRecord>(`/agent-builds/${buildId}`);
}

export async function deleteAgentBuild(buildId: number): Promise<{ success: boolean }> {
  return apiFetch<{ success: boolean }>(`/agent-builds/${buildId}`, { method: 'DELETE' });
}
