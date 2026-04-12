import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
export const useConnectionStore = defineStore('connection', () => {
    const profiles = ref([]);
    const activeProfileId = ref(null);
    const activeProfile = computed(() => profiles.value.find(p => p.id === activeProfileId.value) || null);
    function loadProfiles() {
        const saved = localStorage.getItem('hermes_backend_profiles');
        if (saved) {
            try {
                profiles.value = JSON.parse(saved);
            }
            catch (e) {
                profiles.value = [];
            }
        }
        const savedActive = localStorage.getItem('hermes_active_profile_id');
        if (savedActive) {
            activeProfileId.value = savedActive;
        }
    }
    function saveProfiles() {
        localStorage.setItem('hermes_backend_profiles', JSON.stringify(profiles.value));
        if (activeProfileId.value) {
            localStorage.setItem('hermes_active_profile_id', activeProfileId.value);
        }
        else {
            localStorage.removeItem('hermes_active_profile_id');
        }
    }
    function addProfile(profile) {
        const newProfile = { ...profile, id: Date.now().toString() };
        profiles.value.push(newProfile);
        saveProfiles();
        return newProfile;
    }
    function updateProfile(id, updates) {
        const index = profiles.value.findIndex(p => p.id === id);
        if (index !== -1) {
            profiles.value[index] = { ...profiles.value[index], ...updates };
            saveProfiles();
        }
    }
    function deleteProfile(id) {
        profiles.value = profiles.value.filter(p => p.id !== id);
        if (activeProfileId.value === id) {
            activeProfileId.value = null;
        }
        saveProfiles();
    }
    function setActiveProfile(id) {
        activeProfileId.value = id;
        saveProfiles();
    }
    function normalizeUrl(url) {
        let normalized = url.trim();
        if (!/^https?:\/\//i.test(normalized)) {
            normalized = 'http://' + normalized;
        }
        if (normalized.endsWith('/')) {
            normalized = normalized.slice(0, -1);
        }
        return normalized;
    }
    loadProfiles();
    return {
        profiles,
        activeProfileId,
        activeProfile,
        loadProfiles,
        saveProfiles,
        addProfile,
        updateProfile,
        deleteProfile,
        setActiveProfile,
        normalizeUrl
    };
});
