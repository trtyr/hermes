import { useConnectionStore } from '@/store/connection';
export async function fetchDashboardStats() {
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
