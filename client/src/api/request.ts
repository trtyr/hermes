import { useConnectionStore } from '@/store/connection';

/**
 * Centralized fetch wrapper that:
 * 1. Injects the auth header from the active profile
 * 2. On 401/403 responses, clears the session and redirects to /login
 *
 * Use this for ALL authenticated API calls so session expiry is
 * handled uniformly across the app.
 */
export async function apiFetch<T = unknown>(
  path: string,
  options: RequestInit = {}
): Promise<T> {
  const store = useConnectionStore();
  const profile = store.activeProfile;

  if (!profile) {
    throw new Error('No active backend profile');
  }

  const headers = new Headers(options.headers);
  headers.set('Authorization', `Bearer ${profile.api_token}`);
  if (!headers.has('Content-Type') && !(options.body instanceof FormData)) {
    headers.set('Content-Type', 'application/json');
  }

  const res = await fetch(`${profile.server_url}${path}`, {
    ...options,
    headers,
  });

  if (res.status === 401 || res.status === 403) {
    store.logout();
    // Lazy import to avoid circular dependency with router
    const { router } = await import('@/router');
    router.push('/login');
    throw new Error('Session expired or unauthorized');
  }

  return res.json();
}
