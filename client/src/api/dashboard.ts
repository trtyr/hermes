import { useConnectionStore } from '@/store/connection';

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
  const store = useConnectionStore();
  const profile = store.activeProfile;
  if (!profile) {
    throw new Error('未连接到后端服务器');
  }

  const res = await fetch(`${profile.server_url}/dashboard/stats`, {
    headers: {
      'Authorization': `Bearer ${profile.api_token}`,
      'Content-Type': 'application/json'
    }
  });

  if (!res.ok) {
    if (res.status === 401 || res.status === 403) {
      throw new Error('后端 API Token 无效或已过期');
    }
    throw new Error(`获取统计数据失败: ${res.statusText}`);
  }

  return res.json();
}
