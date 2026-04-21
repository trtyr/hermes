import { useConnectionStore } from '../store/connection';
/**
 * Authenticate against the backend server.
 * POST /auth/login → session token
 */
export async function loginToBackend(serverUrl, username, password) {
    const store = useConnectionStore();
    const url = store.normalizeUrl(serverUrl);
    try {
        const res = await fetch(`${url}/auth/login`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ username, password }),
        });
        const data = await res.json();
        if (!res.ok) {
            if (res.status === 401) {
                return { success: false, error: '用户名或密码错误', errorType: 'auth' };
            }
            return { success: false, error: data.detail || '服务器错误', errorType: 'server' };
        }
        return {
            success: true,
            session_token: data.session_token,
            username: data.username,
            expires_at: data.expires_at,
        };
    }
    catch {
        return { success: false, error: '无法连接到服务器', errorType: 'network' };
    }
}
/**
 * Check if current session is still valid.
 * GET /auth/me
 */
export async function checkSession(serverUrl, sessionToken) {
    const store = useConnectionStore();
    const url = store.normalizeUrl(serverUrl);
    try {
        const res = await fetch(`${url}/auth/me`, {
            headers: { Authorization: `Bearer ${sessionToken}` },
        });
        if (!res.ok)
            return false;
        const data = await res.json();
        return data.authenticated === true;
    }
    catch {
        return false;
    }
}
/**
 * Legacy: test backend connectivity and token validity.
 */
export async function testConnection(serverUrl, apiToken) {
    const store = useConnectionStore();
    const url = store.normalizeUrl(serverUrl);
    try {
        const healthRes = await fetch(`${url}/health`);
        if (!healthRes.ok) {
            return { success: false, errorType: 'network', message: 'Backend is unreachable or returned an error.' };
        }
        const tasksRes = await fetch(`${url}/tasks?limit=1&offset=0`, {
            headers: {
                'Authorization': `Bearer ${apiToken}`,
                'Content-Type': 'application/json'
            }
        });
        if (tasksRes.status === 401 || tasksRes.status === 403) {
            return { success: false, errorType: 'auth', message: 'Backend token is invalid.' };
        }
        if (!tasksRes.ok) {
            return { success: false, errorType: 'server', message: 'Backend is reachable but returned an unexpected error.' };
        }
        return { success: true };
    }
    catch (error) {
        return { success: false, errorType: 'network', message: 'Cannot reach backend address.' };
    }
}
