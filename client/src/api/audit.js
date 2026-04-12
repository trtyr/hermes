import { useConnectionStore } from '@/store/connection';
function getAuthHeaders() {
    const store = useConnectionStore();
    const profile = store.activeProfile;
    if (!profile)
        throw new Error('未连接到后端服务器');
    return {
        'Authorization': `Bearer ${profile.api_token}`,
        'Content-Type': 'application/json'
    };
}
function getBaseUrl() {
    const store = useConnectionStore();
    const profile = store.activeProfile;
    if (!profile)
        throw new Error('未连接到后端服务器');
    return profile.server_url;
}
export async function fetchAudits(filter = {}) {
    const url = new URL(`${getBaseUrl()}/audits`);
    if (filter.operator)
        url.searchParams.append('operator', filter.operator);
    if (filter.action)
        url.searchParams.append('action', filter.action);
    if (filter.target_kind)
        url.searchParams.append('target_kind', filter.target_kind);
    if (filter.target_id)
        url.searchParams.append('target_id', filter.target_id);
    if (filter.limit !== undefined)
        url.searchParams.append('limit', filter.limit.toString());
    if (filter.offset !== undefined)
        url.searchParams.append('offset', filter.offset.toString());
    const res = await fetch(url.toString(), { headers: getAuthHeaders() });
    if (!res.ok)
        throw new Error('获取审计日志失败: ' + res.statusText);
    return res.json();
}
