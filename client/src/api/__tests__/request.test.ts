import { describe, it, expect, beforeEach, vi } from 'vitest';
import { setActivePinia, createPinia } from 'pinia';
import { useConnectionStore } from '@/store/connection';

// Mock the router module to avoid circular dependency during apiFetch error handling
vi.mock('@/router', () => ({
  router: { push: vi.fn() },
}));

// We need to import apiFetch after the mock is set up
import { apiFetch, apiFetchBlob } from '../request';

describe('apiFetch', () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    localStorage.clear();
    vi.restoreAllMocks();
  });

  function createProfile() {
    const store = useConnectionStore();
    const p = store.addProfile({
      connection_name: 'test',
      server_url: 'http://localhost:3000',
      api_token: 'test-token',
    });
    store.setActiveProfile(p.id);
    return p;
  }

  it('throws when no active profile', async () => {
    await expect(apiFetch('/test')).rejects.toThrow('No active backend profile');
  });

  it('sends Authorization header with Bearer token', async () => {
    createProfile();
    const mockFetch = vi.fn().mockResolvedValue({
      ok: true,
      status: 200,
      statusText: 'OK',
      json: () => Promise.resolve({ data: 'ok' }),
    });
    vi.stubGlobal('fetch', mockFetch);

    await apiFetch('/api/test');

    expect(mockFetch).toHaveBeenCalledTimes(1);
    const [url, init] = mockFetch.mock.calls[0] as [string, RequestInit];
    expect(url).toBe('http://localhost:3000/api/test');
    const headers = init.headers as Headers;
    expect(headers.get('Authorization')).toBe('Bearer test-token');
    expect(headers.get('Content-Type')).toBe('application/json');
  });

  it('returns parsed JSON on 200', async () => {
    createProfile();
    const mockJson = { items: [1, 2, 3] };
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue({
      ok: true,
      status: 200,
      json: () => Promise.resolve(mockJson),
    }));

    const result = await apiFetch('/api/agents');
    expect(result).toEqual(mockJson);
  });

  it('returns undefined on 204 No Content', async () => {
    createProfile();
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue({
      ok: true,
      status: 204,
    }));

    const result = await apiFetch('/api/delete');
    expect(result).toBeUndefined();
  });

  it('throws on 401 with session expiry redirect', async () => {
    createProfile();
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue({
      ok: false,
      status: 401,
      statusText: 'Unauthorized',
    }));

    await expect(apiFetch('/api/protected')).rejects.toThrow('Session expired or unauthorized');
    const store = useConnectionStore();
    expect(store.activeProfileId).toBeNull();
  });

  it('throws on 403 with session expiry redirect', async () => {
    createProfile();
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue({
      ok: false,
      status: 403,
      statusText: 'Forbidden',
    }));

    await expect(apiFetch('/api/protected')).rejects.toThrow('Session expired or unauthorized');
  });

  it('passes through non-auth errors without logout', async () => {
    createProfile();
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue({
      ok: false,
      status: 500,
      statusText: 'Internal Server Error',
      json: () => Promise.resolve({ error: 'boom' }),
    }));

    const result = await apiFetch('/api/error');
    expect(result).toEqual({ error: 'boom' });
    const store = useConnectionStore();
    expect(store.activeProfileId).not.toBeNull();
  });

  it('supports custom method and body', async () => {
    createProfile();
    const mockFetch = vi.fn().mockResolvedValue({
      ok: true,
      status: 201,
      json: () => Promise.resolve({ id: 'new' }),
    });
    vi.stubGlobal('fetch', mockFetch);

    const body = JSON.stringify({ name: 'test-listener' });
    await apiFetch('/api/listeners', { method: 'POST', body });

    const [, init] = mockFetch.mock.calls[0] as [string, RequestInit];
    expect(init.method).toBe('POST');
    expect(init.body).toBe(body);
  });
});

describe('apiFetchBlob', () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    localStorage.clear();
    vi.restoreAllMocks();
  });

  function createProfile() {
    const store = useConnectionStore();
    const p = store.addProfile({
      connection_name: 'test',
      server_url: 'http://localhost:3000',
      api_token: 'test-token',
    });
    store.setActiveProfile(p.id);
    return p;
  }

  it('returns a Blob on success', async () => {
    createProfile();
    const mockBlob = new Blob(['binary data']);
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue({
      ok: true,
      status: 200,
      blob: () => Promise.resolve(mockBlob),
    }));

    const result = await apiFetchBlob('/api/download');
    expect(result).toBeInstanceOf(Blob);
  });

  it('throws on 401 redirect', async () => {
    createProfile();
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue({
      ok: false,
      status: 401,
    }));

    await expect(apiFetchBlob('/api/download')).rejects.toThrow('Session expired or unauthorized');
  });
});
