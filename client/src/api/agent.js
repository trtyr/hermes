import { useConnectionStore } from '@/store/connection';
export async function fetchAgents(params = {}) {
    const store = useConnectionStore();
    const profile = store.activeProfile;
    if (!profile)
        throw new Error('未连接到后端服务器');
    const url = new URL(`${profile.server_url}/agents/history`);
    if (params.limit !== undefined)
        url.searchParams.append('limit', params.limit.toString());
    if (params.offset !== undefined)
        url.searchParams.append('offset', params.offset.toString());
    if (params.keyword)
        url.searchParams.append('keyword', params.keyword);
    if (params.online !== undefined)
        url.searchParams.append('online', params.online.toString());
    if (params.disabled !== undefined)
        url.searchParams.append('disabled', params.disabled.toString());
    if (params.tag)
        url.searchParams.append('tag', params.tag);
    const res = await fetch(url.toString(), {
        headers: {
            'Authorization': `Bearer ${profile.api_token}`,
            'Content-Type': 'application/json'
        }
    });
    if (!res.ok)
        throw new Error(`获取节点列表失败: ${res.statusText}`);
    return res.json();
}
export async function fetchAgentDetail(agentId) {
    const store = useConnectionStore();
    const profile = store.activeProfile;
    if (!profile)
        throw new Error('未连接到后端服务器');
    const res = await fetch(`${profile.server_url}/agents/${agentId}`, {
        headers: {
            'Authorization': `Bearer ${profile.api_token}`,
            'Content-Type': 'application/json'
        }
    });
    if (!res.ok)
        throw new Error(`获取节点详情失败: ${res.statusText}`);
    return res.json();
}
export async function updateBeaconConfig(agentId, sleep_interval, jitter) {
    const store = useConnectionStore();
    const profile = store.activeProfile;
    if (!profile)
        throw new Error('未连接到后端服务器');
    const res = await fetch(`${profile.server_url}/agents/${agentId}/beacon-config`, {
        method: 'POST',
        headers: {
            'Authorization': `Bearer ${profile.api_token}`,
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ sleep_interval, jitter })
    });
    if (!res.ok)
        throw new Error(`更新 Beacon 配置失败: ${res.statusText}`);
    return res.json();
}
export async function dispatchTask(agentId, payload) {
    const store = useConnectionStore();
    const profile = store.activeProfile;
    if (!profile)
        throw new Error('未连接到后端服务器');
    const res = await fetch(`${profile.server_url}/agents/${agentId}/tasks`, {
        method: 'POST',
        headers: {
            'Authorization': `Bearer ${profile.api_token}`,
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(payload)
    });
    if (!res.ok)
        throw new Error(`下发任务失败: ${res.statusText}`);
    return res.json();
}
export async function disconnectAgent(agentId) {
    return executeAgentAction(agentId, 'disconnect');
}
export async function disableAgent(agentId) {
    return executeAgentAction(agentId, 'disable');
}
export async function enableAgent(agentId) {
    return executeAgentAction(agentId, 'enable');
}
export async function deleteAgent(agentId) {
    const store = useConnectionStore();
    const profile = store.activeProfile;
    if (!profile)
        throw new Error('未连接到后端服务器');
    const res = await fetch(`${profile.server_url}/agents/${agentId}`, {
        method: 'DELETE',
        headers: {
            'Authorization': `Bearer ${profile.api_token}`
        }
    });
    if (!res.ok)
        throw new Error(`删除节点失败: ${res.statusText}`);
    return res.json();
}
export async function uploadFile(agentId, remotePath, contentBase64) {
    const store = useConnectionStore();
    const profile = store.activeProfile;
    if (!profile)
        throw new Error('未连接到后端服务器');
    const res = await fetch(`${profile.server_url}/agents/${agentId}/upload`, {
        method: 'POST',
        headers: {
            'Authorization': `Bearer ${profile.api_token}`,
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ remote_path: remotePath, content_base64: contentBase64 })
    });
    if (!res.ok)
        throw new Error(`上传文件失败: ${res.statusText}`);
    return res.json();
}
export async function downloadFile(agentId, remotePath) {
    const store = useConnectionStore();
    const profile = store.activeProfile;
    if (!profile)
        throw new Error('未连接到后端服务器');
    const res = await fetch(`${profile.server_url}/agents/${agentId}/download`, {
        method: 'POST',
        headers: {
            'Authorization': `Bearer ${profile.api_token}`,
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ remote_path: remotePath })
    });
    if (!res.ok)
        throw new Error(`下载文件失败: ${res.statusText}`);
    return res.json();
}
export async function takeScreenshot(agentId) {
    const store = useConnectionStore();
    const profile = store.activeProfile;
    if (!profile)
        throw new Error('未连接到后端服务器');
    const res = await fetch(`${profile.server_url}/agents/${agentId}/screenshot`, {
        method: 'POST',
        headers: {
            'Authorization': `Bearer ${profile.api_token}`,
            'Content-Type': 'application/json'
        }
    });
    if (!res.ok)
        throw new Error(`截图失败: ${res.statusText}`);
    return res.json();
}
export async function updateAgentTags(agentId, tags) {
    const store = useConnectionStore();
    const profile = store.activeProfile;
    if (!profile)
        throw new Error('未连接到后端服务器');
    const res = await fetch(`${profile.server_url}/agents/${agentId}`, {
        method: 'PATCH',
        headers: {
            'Authorization': `Bearer ${profile.api_token}`,
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ tags })
    });
    if (!res.ok)
        throw new Error(`更新标签失败: ${res.statusText}`);
    return res.json();
}
async function executeAgentAction(agentId, action) {
    const store = useConnectionStore();
    const profile = store.activeProfile;
    if (!profile)
        throw new Error('未连接到后端服务器');
    const res = await fetch(`${profile.server_url}/agents/${agentId}/${action}`, {
        method: 'POST',
        headers: {
            'Authorization': `Bearer ${profile.api_token}`,
            'Content-Type': 'application/json'
        }
    });
    if (!res.ok)
        throw new Error(`执行操作 ${action} 失败: ${res.statusText}`);
    return res.json();
}
