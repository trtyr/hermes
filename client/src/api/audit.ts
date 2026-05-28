import { apiFetch } from './request';

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

export async function fetchAudits(filter: AuditFilter = {}): Promise<AuditListResponse> {
  const searchParams = new URLSearchParams();
  if (filter.operator) searchParams.append('operator', filter.operator);
  if (filter.action) searchParams.append('action', filter.action);
  if (filter.target_kind) searchParams.append('target_kind', filter.target_kind);
  if (filter.target_id) searchParams.append('target_id', filter.target_id);
  if (filter.limit !== undefined) searchParams.append('limit', filter.limit.toString());
  if (filter.offset !== undefined) searchParams.append('offset', filter.offset.toString());

  const qs = searchParams.toString();
  return apiFetch<AuditListResponse>(`/audits${qs ? `?${qs}` : ''}`);
}
