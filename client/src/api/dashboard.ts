import { apiFetch } from './request';

export interface DashboardStats {
  generated_at: number;
  server: {
    hostname: string | null;
    os_name: string | null;
    os_version: string | null;
    kernel_version: string | null;
    architecture: string | null;
    uptime_seconds: number;
    cpu_cores: number | null;
    load_average: {
      one: number;
      five: number;
      fifteen: number;
    };
    memory: {
      total_bytes: number;
      used_bytes: number;
      available_bytes: number;
    };
  };
  agents: {
    total: number;
    online: number;
    offline: number;
    disabled: number;
    connected_sessions: number;
  };
  listeners: {
    total: number;
    enabled: number;
    disabled: number;
    running: number;
    stopped: number;
    starting: number;
    error: number;
    by_kind: {
      tcp_json: number;
      https_json: number;
      private_proto: number;
    };
  };
}

export async function fetchDashboardStats(): Promise<DashboardStats> {
  return apiFetch<DashboardStats>('/dashboard/stats');
}
