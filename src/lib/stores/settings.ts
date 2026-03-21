import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface AlertSettings {
  enabled: boolean;
  low_threshold: number;
  high_threshold: number;
  notify_on_low: boolean;
  notify_on_high: boolean;
}

/**
 * Preferences for fullscreen display configuration.
 * Controls visibility of chart and stats elements in fullscreen mode.
 */
export interface FullscreenPreferences {
  showChart: boolean;
  showStats: boolean;
  // BPM always visible per design decision
}

/**
 * @deprecated Use FullscreenPreferences instead for granular control.
 * Preset modes are being replaced with checkbox toggles.
 */
export type FullscreenMode = 'simple' | 'standard' | 'stats' | 'chart';

export interface SettingsState {
  alertSettings: AlertSettings;
  sessionId: string;
  fullscreenMode: FullscreenMode;
  fullscreenPreferences: FullscreenPreferences;
}

function createSettingsStore() {
  const defaultAlertSettings: AlertSettings = {
    enabled: true,
    low_threshold: 50,
    high_threshold: 180,
    notify_on_low: true,
    notify_on_high: true,
  };

  const initialState: SettingsState = {
    alertSettings: defaultAlertSettings,
    sessionId: generateSessionId(),
    fullscreenMode: 'standard',
    fullscreenPreferences: {
      showChart: true,
      showStats: true,
    },
  };

  const { subscribe, update, set } = writable<SettingsState>(initialState);

  // Load settings from backend - call this in onMount
  async function loadSettings() {
    if (typeof window === 'undefined') return;
    try {
      const settings = await invoke<AlertSettings>('get_alert_settings');
      update((state) => ({
        ...state,
        alertSettings: settings,
      }));
    } catch (error) {
      console.error('Failed to load alert settings:', error);
    }
  }

  // Save settings to backend
  async function saveSettings(settings: AlertSettings) {
    if (typeof window === 'undefined') return;
    try {
      await invoke('set_alert_settings', { settings });
      update((state) => ({
        ...state,
        alertSettings: settings,
      }));
    } catch (error) {
      console.error('Failed to save alert settings:', error);
    }
  }

  return {
    subscribe,
    loadSettings,
    updateAlertSettings: (settings: AlertSettings) => saveSettings(settings),
    setFullscreenMode: (mode: FullscreenMode) => {
      update((state) => ({
        ...state,
        fullscreenMode: mode,
      }));
    },
    setFullscreenPreferences: (prefs: Partial<FullscreenPreferences>) => {
      update((state) => ({
        ...state,
        fullscreenPreferences: { ...state.fullscreenPreferences, ...prefs },
      }));
    },
    newSession: () => {
      update((state) => ({
        ...state,
        sessionId: generateSessionId(),
      }));
    },
    reset: () => set(initialState),
  };
}

function generateSessionId(): string {
  return `session_${Date.now()}_${Math.random().toString(36).substring(2, 11)}`;
}

export const settings = createSettingsStore();