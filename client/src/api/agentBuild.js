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
export async function fetchAgentBuilds(params = {}) {
    const url = new URL(`${getBaseUrl()}/agent-builds`);
    if (params.status)
        url.searchParams.append('status', params.status);
    if (params.target_triple)
        url.searchParams.append('target_triple', params.target_triple);
    if (params.limit !== undefined)
        url.searchParams.append('limit', params.limit.toString());
    if (params.offset !== undefined)
        url.searchParams.append('offset', params.offset.toString());
    const res = await fetch(url.toString(), { headers: getAuthHeaders() });
    if (!res.ok)
        throw new Error('获取构建列表失败: ' + res.statusText);
    return res.json();
}
export async function createAgentBuild(data) {
    const res = await fetch(`${getBaseUrl()}/agent-builds`, {
        method: 'POST',
        headers: getAuthHeaders(),
        body: JSON.stringify(data)
    });
    if (!res.ok)
        throw new Error('创建构建失败: ' + res.statusText);
    return res.json();
}
export async function fetchAgentBuild(buildId) {
    const res = await fetch(`${getBaseUrl()}/agent-builds/${buildId}`, { headers: getAuthHeaders() });
    if (!res.ok)
        throw new Error('获取构建详情失败: ' + res.statusText);
    return res.json();
}
export function getBuildDownloadUrl(buildId) {
    return `${getBaseUrl()}/agent-builds/${buildId}/download`;
}
