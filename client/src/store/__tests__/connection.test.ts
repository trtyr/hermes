import { describe, it, expect, beforeEach, vi } from 'vitest';
import { setActivePinia, createPinia } from 'pinia';
import { useConnectionStore } from '../connection';

describe('useConnectionStore', () => {
  beforeEach(() => {
    // Reset Pinia before each test
    setActivePinia(createPinia());
    // Clear localStorage
    localStorage.clear();
  });

  describe('addProfile', () => {
    it('adds a profile to the array', () => {
      const store = useConnectionStore();
      expect(store.profiles).toHaveLength(0);

      const profile = store.addProfile({
        connection_name: 'test-server',
        server_url: 'http://localhost:3000',
        api_token: 'token123',
      });

      expect(store.profiles).toHaveLength(1);
      expect(profile.connection_name).toBe('test-server');
      expect(profile.server_url).toBe('http://localhost:3000');
      expect(profile.api_token).toBe('token123');
      expect(profile.id).toBeTruthy();
    });

    it('generates unique IDs', async () => {
      const store = useConnectionStore();
      const p1 = store.addProfile({
        connection_name: 's1',
        server_url: 'http://localhost:3001',
        api_token: 't1',
      });
      // Ensure different millisecond for ID generation
      await new Promise((r) => setTimeout(r, 2));
      const p2 = store.addProfile({
        connection_name: 's2',
        server_url: 'http://localhost:3002',
        api_token: 't2',
      });

      expect(p1.id).not.toBe(p2.id);
      expect(store.profiles).toHaveLength(2);
    });

    it('persists to localStorage', () => {
      const store = useConnectionStore();
      store.addProfile({
        connection_name: 'persisted',
        server_url: 'http://example.com',
        api_token: 'ptoken',
      });

      const saved = localStorage.getItem('hermes_backend_profiles');
      expect(saved).toBeTruthy();
      const parsed = JSON.parse(saved!);
      expect(parsed).toHaveLength(1);
      expect(parsed[0].connection_name).toBe('persisted');
    });
  });

  describe('setActiveProfile', () => {
    it('sets active profile ID', () => {
      const store = useConnectionStore();
      store.setActiveProfile('profile-1');
      expect(store.activeProfileId).toBe('profile-1');
    });

    it('clears active profile when null', () => {
      const store = useConnectionStore();
      store.setActiveProfile('profile-1');
      store.setActiveProfile(null);
      expect(store.activeProfileId).toBeNull();
    });

    it('persists active profile ID to localStorage', () => {
      const store = useConnectionStore();
      store.setActiveProfile('profile-x');
      expect(localStorage.getItem('hermes_active_profile_id')).toBe('profile-x');
    });
  });

  describe('activeProfile computed', () => {
    it('returns profile matching active ID', () => {
      const store = useConnectionStore();
      const p = store.addProfile({
        connection_name: 'target',
        server_url: 'http://target.com',
        api_token: 't',
      });
      store.setActiveProfile(p.id);
      expect(store.activeProfile).toEqual(p);
    });

    it('returns null when no matching profile', () => {
      const store = useConnectionStore();
      store.setActiveProfile('nonexistent');
      expect(store.activeProfile).toBeNull();
    });
  });

  describe('deleteProfile', () => {
    it('removes profile from array', () => {
      const store = useConnectionStore();
      const p = store.addProfile({
        connection_name: 'todelete',
        server_url: 'http://del.com',
        api_token: 'd',
      });
      expect(store.profiles).toHaveLength(1);

      store.deleteProfile(p.id);
      expect(store.profiles).toHaveLength(0);
    });

    it('clears active profile when deleted profile was active', () => {
      const store = useConnectionStore();
      const p = store.addProfile({
        connection_name: 'active-del',
        server_url: 'http://ad.com',
        api_token: 'ad',
      });
      store.setActiveProfile(p.id);
      store.deleteProfile(p.id);

      expect(store.activeProfileId).toBeNull();
    });
  });

  describe('updateProfile', () => {
    it('updates profile fields', () => {
      const store = useConnectionStore();
      const p = store.addProfile({
        connection_name: 'old-name',
        server_url: 'http://old.com',
        api_token: 'old',
      });

      store.updateProfile(p.id, { connection_name: 'new-name' });
      expect(store.profiles[0].connection_name).toBe('new-name');
      expect(store.profiles[0].server_url).toBe('http://old.com');
    });
  });

  describe('logout', () => {
    it('clears activeProfileId', () => {
      const store = useConnectionStore();
      store.setActiveProfile('some-id');
      store.logout();
      expect(store.activeProfileId).toBeNull();
    });

    it('keeps saved profiles', () => {
      const store = useConnectionStore();
      store.addProfile({
        connection_name: 'keep',
        server_url: 'http://keep.com',
        api_token: 'k',
      });
      store.setActiveProfile(store.profiles[0].id);
      store.logout();

      expect(store.profiles).toHaveLength(1);
      expect(store.activeProfileId).toBeNull();
    });
  });

  describe('normalizeUrl', () => {
    it('adds http:// when no scheme', () => {
      const store = useConnectionStore();
      expect(store.normalizeUrl('localhost:3000')).toBe('http://localhost:3000');
    });

    it('keeps existing https://', () => {
      const store = useConnectionStore();
      expect(store.normalizeUrl('https://example.com')).toBe('https://example.com');
    });

    it('strips trailing slash', () => {
      const store = useConnectionStore();
      expect(store.normalizeUrl('http://localhost:3000/')).toBe('http://localhost:3000');
    });

    it('trims whitespace', () => {
      const store = useConnectionStore();
      expect(store.normalizeUrl('  http://foo.com  ')).toBe('http://foo.com');
    });
  });

  describe('loadProfiles', () => {
    it('loads profiles from localStorage', () => {
      const savedProfiles = [
        { id: 'p1', connection_name: 'saved', server_url: 'http://s.com', api_token: 'st' },
      ];
      localStorage.setItem('hermes_backend_profiles', JSON.stringify(savedProfiles));
      localStorage.setItem('hermes_active_profile_id', 'p1');

      const store = useConnectionStore();
      expect(store.profiles).toHaveLength(1);
      expect(store.activeProfileId).toBe('p1');
    });

    it('handles corrupt localStorage gracefully', () => {
      localStorage.setItem('hermes_backend_profiles', 'invalid json');
      const store = useConnectionStore();
      expect(store.profiles).toHaveLength(0);
    });
  });
});
