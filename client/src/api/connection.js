import { useConnectionStore } from '../store/connection';
export async function testConnection(serverUrl, apiToken) {
    const store = useConnectionStore();
    const url = store.normalizeUrl(serverUrl);
    try {
        // Step 1: Reachability check
        const healthRes = await fetch(`${url}/health`);
        if (!healthRes.ok) {
            return { success: false, errorType: 'network', message: 'Backend is unreachable or returned an error.' };
        }
        // Step 2: Token validation check
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
