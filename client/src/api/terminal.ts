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
  const store = useConnectionStore();
  const profile = store.activeProfile;
  if (!profile) throw new Error('未连接到后端服务器');

  const res = await fetch(`${profile.server_url}/web/terminal/open`, {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${profile.api_token}`,
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({ agent_id: agentId })
  });

  if (!res.ok) throw new Error(`无法打开终端会话: ${res.statusText}`);
  return res.json();
}

export async function submitTerminalCommand(sessionId: string, line: string): Promise<{ success: boolean, message: string, data: TerminalCommandResponse }> {
  const store = useConnectionStore();
  const profile = store.activeProfile;
  if (!profile) throw new Error('未连接到后端服务器');

  const res = await fetch(`${profile.server_url}/web/terminal/command`, {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${profile.api_token}`,
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({ session_id: sessionId, line })
  });

  if (!res.ok) throw new Error(`命令提交失败: ${res.statusText}`);
  return res.json();
}

export async function closeTerminalSession(sessionId: string): Promise<any> {
  const store = useConnectionStore();
  const profile = store.activeProfile;
  if (!profile) throw new Error('未连接到后端服务器');

  const res = await fetch(`${profile.server_url}/web/terminal/close`, {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${profile.api_token}`,
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({ session_id: sessionId })
  });

  if (!res.ok) throw new Error(`关闭终端会话失败: ${res.statusText}`);
  return res.json();
}

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
