import { apiFetch } from './request';

export interface ListenerRecord {
  listener_id: number;
  name: string;
  kind: string;
  bind_host: string;
  bind_port: number;
  enabled: boolean;
  config: Record<string, unknown>;
  runtime_status: string;
  last_error: string | null;
  created_at: number;
  updated_at: number;
}

export interface SpawnListenerRequest {
  name: string;
  kind: string;
  bind_host: string;
  bind_port: number;
}

export async function fetchListeners(): Promise<{ success: boolean; listeners: ListenerRecord[] }> {
  return apiFetch<{ success: boolean; listeners: ListenerRecord[] }>('/listeners');
}

export async function spawnListener(data: SpawnListenerRequest): Promise<{ success: boolean; listener_id: string }> {
  return apiFetch<{ success: boolean; listener_id: string }>('/listeners', {
    method: 'POST',
    body: JSON.stringify(data),
  });
}

export async function startListener(id: number): Promise<{ success: boolean }> {
  return apiFetch<{ success: boolean }>(`/listeners/${id}/enable`, { method: 'POST' });
}

export async function stopListener(id: number): Promise<{ success: boolean }> {
  return apiFetch<{ success: boolean }>(`/listeners/${id}/disable`, { method: 'POST' });
}

export async function deleteListener(id: number): Promise<{ success: boolean }> {
  return apiFetch<{ success: boolean }>(`/listeners/${id}`, { method: 'DELETE' });
}
