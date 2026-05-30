import { useConnectionStore } from '@/store/connection';

/**
 * Build headers with auth token from the active profile.
 * Throws if no profile is active.
 */
function buildAuthHeaders(extra?: HeadersInit): Headers {
  const store = useConnectionStore();
  const profile = store.activeProfile;

  if (!profile) {
    throw new Error('No active backend profile');
  }

  const headers = new Headers(extra);
  headers.set('Authorization', `Bearer ${profile.api_token}`);
  if (!headers.has('Content-Type') && !(extra instanceof FormData)) {
    headers.set('Content-Type', 'application/json');
  }
  return headers;
}

/**
 * Resolve the full URL from a path using the active profile's server_url.
 */
function resolveUrl(path: string): string {
  const store = useConnectionStore();
  const profile = store.activeProfile;

  if (!profile) {
    throw new Error('No active backend profile');
  }

  return `${profile.server_url}${path}`;
}

/**
 * Handle 401/403 by clearing session and redirecting to /login.
 * Returns true if the response was unauthorized (caller should throw).
 */
async function handleAuthError(res: Response): Promise<boolean> {
  if (res.status === 401 || res.status === 403) {
    const store = useConnectionStore();
    store.logout();
    // Lazy import to avoid circular dependency with router
    const { router } = await import('@/router');
    router.push('/login');
    throw new Error('Session expired or unauthorized');
  }
  return false;
}

/**
 * Centralized fetch wrapper that:
 * 1. Injects the auth header from the active profile
 * 2. On 401/403 responses, clears the session and redirects to /login
 * 3. On 204 No Content, returns undefined without calling .json()
 *
 * Use this for ALL authenticated JSON API calls so session expiry is
 * handled uniformly across the app.
 */
export async function apiFetch<T = unknown>(
  path: string,
  options: RequestInit = {}
): Promise<T> {
  const headers = buildAuthHeaders(options.headers);

  const res = await fetch(resolveUrl(path), {
    ...options,
    headers,
  });

  await handleAuthError(res);

  if (res.status === 204) {
    return undefined as T;
  }

  const body = await res.json();

  if (!res.ok) {
    const detail = body?.detail || body?.message || `请求失败 (${res.status})`;
    throw new Error(detail);
  }

  return body as T;
}

/**
 * Fetch wrapper for blob responses (file downloads).
 * Injects auth header, handles 401/403, returns the response as a Blob.
 */
export async function apiFetchBlob(
  path: string,
  options: RequestInit = {}
): Promise<Blob> {
  const headers = buildAuthHeaders(options.headers);

  const res = await fetch(resolveUrl(path), {
    ...options,
    headers,
  });

  await handleAuthError(res);

  return res.blob();
}
