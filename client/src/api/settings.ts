import { apiFetch } from './request';

export interface AgentAuthSettings {
  agent_token: string;
  agent_auth_mode: string;
}

export async function getAuthSettings(): Promise<AgentAuthSettings> {
  return apiFetch<AgentAuthSettings>('/server/auth-settings');
}

export async function updateAuthSettings(settings: AgentAuthSettings): Promise<{ success: boolean; detail: string }> {
  return apiFetch<{ success: boolean; detail: string }>('/server/auth-settings', {
    method: 'PUT',
    body: JSON.stringify(settings),
  });
}
