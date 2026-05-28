import { apiFetch } from './request';
import { useConnectionStore } from '@/store/connection';

export interface TerminalSessionResponse {
  session_id: string;
  cwd: string;
  status: string;
}

export interface TerminalCommandResponse {
  session_id: string;
  command_id: string;
  state: string;
}

export async function openTerminalSession(agentId: string): Promise<{ success: boolean, message: string, data: TerminalSessionResponse }> {
  return apiFetch<{ success: boolean, message: string, data: TerminalSessionResponse }>('/web/terminal/open', {
    method: 'POST',
    body: JSON.stringify({ agent_id: agentId }),
  });
}

export async function submitTerminalCommand(sessionId: string, line: string): Promise<{ success: boolean, message: string, data: TerminalCommandResponse }> {
  return apiFetch<{ success: boolean, message: string, data: TerminalCommandResponse }>('/web/terminal/command', {
    method: 'POST',
    body: JSON.stringify({ session_id: sessionId, line }),
  });
}

export async function closeTerminalSession(sessionId: string): Promise<{ success: boolean }> {
  return apiFetch<{ success: boolean }>('/web/terminal/close', {
    method: 'POST',
    body: JSON.stringify({ session_id: sessionId }),
  });
}

/**
 * Build the WebSocket URL for terminal sessions.
 * This is not an HTTP fetch — it constructs a ws:// or wss:// URL
 * with the api_token as a query parameter for WS auth.
 */
export function buildTerminalWsUrl(): string {
  const store = useConnectionStore();
  const profile = store.activeProfile;
  if (!profile) throw new Error('未连接到后端服务器');

  const httpUrl = new URL(profile.server_url);
  const wsProtocol = httpUrl.protocol === 'https:' ? 'wss:' : 'ws:';
  const wsUrl = new URL('/web/terminal/ws', `${wsProtocol}//${httpUrl.host}`);
  wsUrl.searchParams.set('api_token', profile.api_token);
  return wsUrl.toString();
}
