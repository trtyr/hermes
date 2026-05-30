import { apiFetch } from './request';

export interface Agent {
  agent_id: string;
  session_id: number | null;
  listener_id: number;
  listener_name: string;
  hostname: string;
  username: string;
  os: string;
  arch: string;
  pid: number;
  internal_ip: string;
  external_ip: string;
  privilege: string;
  tags: string[];
  sleep_interval: number;
  jitter: number;
  peer_addr: string;
  connected_at: number;
  last_seen: number;
  is_online: boolean;
  is_disabled: boolean;
  updated_at: number;
}

export interface AgentListResponse {
  agents: Agent[];
  total: number;
  limit: number;
  offset: number;
}

export async function fetchAgents(params: { limit?: number, offset?: number, keyword?: string, online?: boolean, disabled?: boolean, tag?: string } = {}): Promise<AgentListResponse> {
  const searchParams = new URLSearchParams();
  if (params.limit !== undefined) searchParams.append('limit', params.limit.toString());
  if (params.offset !== undefined) searchParams.append('offset', params.offset.toString());
  if (params.keyword) searchParams.append('keyword', params.keyword);
  if (params.online !== undefined) searchParams.append('online', params.online.toString());
  if (params.disabled !== undefined) searchParams.append('disabled', params.disabled.toString());
  if (params.tag) searchParams.append('tag', params.tag);

  const qs = searchParams.toString();
  return apiFetch<AgentListResponse>(`/agents/history${qs ? `?${qs}` : ''}`);
}

export async function fetchAgentDetail(agentId: string): Promise<Agent> {
  return apiFetch<Agent>(`/agents/${agentId}`);
}

export async function updateBeaconConfig(agentId: string, sleep_interval: number, jitter: number): Promise<{ success: boolean, agent: Agent }> {
  return apiFetch<{ success: boolean, agent: Agent }>(`/agents/${agentId}/beacon-config`, {
    method: 'POST',
    body: JSON.stringify({ sleep_interval, jitter }),
  });
}

export async function dispatchTask(agentId: string, payload: Record<string, unknown>): Promise<{ success: boolean; task_id: string }> {
  return apiFetch<{ success: boolean; task_id: string }>(`/agents/${agentId}/tasks`, {
    method: 'POST',
    body: JSON.stringify(payload),
  });
}

export async function disconnectAgent(agentId: string): Promise<{ success: boolean }> {
  return apiFetch<{ success: boolean }>(`/agents/${agentId}/disconnect`, { method: 'POST' });
}

export async function disableAgent(agentId: string): Promise<{ success: boolean }> {
  return apiFetch<{ success: boolean }>(`/agents/${agentId}/disable`, { method: 'POST' });
}

export async function enableAgent(agentId: string): Promise<{ success: boolean }> {
  return apiFetch<{ success: boolean }>(`/agents/${agentId}/enable`, { method: 'POST' });
}

export async function deleteAgent(agentId: string): Promise<{ success: boolean }> {
  return apiFetch<{ success: boolean }>(`/agents/${agentId}`, { method: 'DELETE' });
}

export async function uploadFile(agentId: string, remotePath: string, contentBase64: string): Promise<{ success: boolean; detail: string; task_id?: string }> {
  return apiFetch<{ success: boolean; detail: string; task_id?: string }>(`/agents/${agentId}/upload`, {
    method: 'POST',
    body: JSON.stringify({ remote_path: remotePath, content_base64: contentBase64 }),
  });
}

export async function downloadFile(agentId: string, remotePath: string): Promise<{ success: boolean; detail: string; task_id?: string }> {
  return apiFetch<{ success: boolean; detail: string; task_id?: string }>(`/agents/${agentId}/download`, {
    method: 'POST',
    body: JSON.stringify({ remote_path: remotePath }),
  });
}

export interface FileEntry {
  name: string;
  is_dir: boolean;
  size: number;
  modified: number;
}

export async function browseFile(agentId: string, path: string): Promise<{ success: boolean; detail: string; task_id?: string }> {
  return apiFetch<{ success: boolean; detail: string; task_id?: string }>(`/agents/${agentId}/browse`, {
    method: 'POST',
    body: JSON.stringify({ path }),
  });
}

export async function takeScreenshot(agentId: string): Promise<{ success: boolean; detail: string; task_id?: string }> {
  return apiFetch<{ success: boolean; detail: string; task_id?: string }>(`/agents/${agentId}/screenshot`, {
    method: 'POST',
  });
}

export async function updateAgentTags(agentId: string, tags: string[]): Promise<{ success: boolean; detail: string }> {
  return apiFetch<{ success: boolean; detail: string }>(`/agents/${agentId}`, {
    method: 'PATCH',
    body: JSON.stringify({ tags }),
  });
}

// ─── Task history ───────────────────────────────────────────────────────────

export interface TaskRecord {
  task_id: string;
  parent_task_id: string | null;
  target_agent_id: string | null;
  command: string;
  payload: string | null;
  status: string;
  created_at: number;
  updated_at: number;
  success: boolean | null;
  output: string | null;
  children: string[];
}

export interface TaskListResponse {
  tasks: TaskRecord[];
  total: number;
  limit: number;
  offset: number;
}

export async function listTasks(params: { agent_id?: string; command?: string; status?: string; limit?: number; offset?: number } = {}): Promise<TaskListResponse> {
  const searchParams = new URLSearchParams();
  if (params.agent_id) searchParams.append('agent_id', params.agent_id);
  if (params.command) searchParams.append('command', params.command);
  if (params.status) searchParams.append('status', params.status);
  if (params.limit !== undefined) searchParams.append('limit', params.limit.toString());
  if (params.offset !== undefined) searchParams.append('offset', params.offset.toString());
  const qs = searchParams.toString();
  return apiFetch<TaskListResponse>(`/tasks${qs ? `?${qs}` : ''}`);
}
