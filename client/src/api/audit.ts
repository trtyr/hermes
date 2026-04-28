import { useConnectionStore } from '@/store/connection';

export interface AuditRecord {
  audit_id: number;
  operator: string;
  action: string;
  target_kind: string;
  target_id: string | null;
  detail: string | null;
  created_at: number;
}

export interface AuditListResponse {
  audits: AuditRecord[];
  total: number;
  limit: number;
  offset: number;
}

export interface AuditFilter {
  operator?: string;
  action?: string;
  target_kind?: string;
  target_id?: string;
  limit?: number;
  offset?: number;
}

function getAuthHeaders() {
  const store = useConnectionStore();
  const profile = store.activeProfile;
  if (!profile) throw new Error('未连接到后端服务器');
  return {
    'x-api-token': profile.api_token,
    'Content-Type': 'application/json'
  };
}

function getBaseUrl() {
  const store = useConnectionStore();
  const profile = store.activeProfile;
  if (!profile) throw new Error('未连接到后端服务器');
  return profile.server_url;
}

export async function fetchAudits(filter: AuditFilter = {}): Promise<AuditListResponse> {
  const url = new URL(`${getBaseUrl()}/audits`);
  if (filter.operator) url.searchParams.append('operator', filter.operator);
  if (filter.action) url.searchParams.append('action', filter.action);
  if (filter.target_kind) url.searchParams.append('target_kind', filter.target_kind);
  if (filter.target_id) url.searchParams.append('target_id', filter.target_id);
  if (filter.limit !== undefined) url.searchParams.append('limit', filter.limit.toString());
  if (filter.offset !== undefined) url.searchParams.append('offset', filter.offset.toString());

  const res = await fetch(url.toString(), { headers: getAuthHeaders() });
  if (!res.ok) throw new Error('获取审计日志失败: ' + res.statusText);
  return res.json();
}
