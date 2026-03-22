<script lang="ts">
  import { settings } from '$lib/stores/settings';
  import ExerciseTypesEditor from './ExerciseTypesEditor.svelte';

  let localSettings = { ...$settings.alertSettings };
  let localPrefs = { ...$settings.fullscreenPreferences };

  $: settingsUpdated = JSON.stringify(localSettings) !== JSON.stringify($settings.alertSettings);

  function saveSettings() {
    settings.updateAlertSettings(localSettings);
  }

  function resetToDefaults() {
    localSettings = {
      enabled: true,
      low_threshold: 50,
      high_threshold: 180,
      notify_on_low: true,
      notify_on_high: true,
    };
    localPrefs = {
      showChart: true,
      showStats: true,
    };
    settings.setFullscreenPreferences(localPrefs);
    saveSettings();
  }

  // Sync local settings when store updates
  $: if ($settings.alertSettings) {
    localSettings = { ...$settings.alertSettings };
  }

  $: if ($settings.fullscreenPreferences) {
    localPrefs = { ...$settings.fullscreenPreferences };
  }

  // Persist fullscreen preference changes immediately
  $: if (JSON.stringify(localPrefs) !== JSON.stringify($settings.fullscreenPreferences)) {
    settings.setFullscreenPreferences(localPrefs);
  }
</script>

<div class="settings-container">
  <!-- Fullscreen Display Settings -->
  <div class="settings-section glass-card">
    <div class="section-header">
      <div class="header-left">
        <svg viewBox="0 0 24 24" fill="currentColor" width="24" height="24">
          <path d="M7 14H5v5h5v-2H7v-3zm-2-4h2V7h3V5H5v5zm12 7h-3v2h5v-5h-2v3zM14 5v2h3v3h2V5h-5z"/>
        </svg>
        <h3>Fullscreen Display</h3>
      </div>
    </div>

    <div class="fullscreen-options">
      <!-- Show Chart Toggle -->
      <div class="toggle-setting">
        <div class="setting-info">
          <span class="setting-label">Show Chart</span>
          <span class="setting-desc">Display heart rate curve in fullscreen mode</span>
        </div>
        <label class="toggle-switch">
          <input type="checkbox" bind:checked={localPrefs.showChart} />
          <span class="toggle-slider"></span>
        </label>
      </div>

      <!-- Show Stats Toggle -->
      <div class="toggle-setting">
        <div class="setting-info">
          <span class="setting-label">Show Stats</span>
          <span class="setting-desc">Display min/avg/max statistics in fullscreen mode</span>
        </div>
        <label class="toggle-switch">
          <input type="checkbox" bind:checked={localPrefs.showStats} />
          <span class="toggle-slider"></span>
        </label>
      </div>

      <p class="bpm-note">BPM display is always visible in fullscreen mode</p>
    </div>
  </div>

  <!-- Alert Settings -->
  <div class="settings-section glass-card">
    <div class="section-header">
      <div class="header-left">
        <svg viewBox="0 0 24 24" fill="currentColor" width="24" height="24">
          <path d="M12 22c1.1 0 2-.9 2-2h-4c0 1.1.89 2 2 2zm6-6v-5c0-3.07-1.64-5.64-4.5-6.32V4c0-.83-.67-1.5-1.5-1.5s-1.5.67-1.5 1.5v.68C7.63 5.36 6 7.92 6 11v5l-2 2v1h16v-1l-2-2z"/>
        </svg>
        <h3>Alert Settings</h3>
      </div>
      <button class="btn-reset" on:click={resetToDefaults}>
        <svg viewBox="0 0 24 24" fill="currentColor" width="14" height="14">
          <path d="M17.65 6.35A7.958 7.958 0 0 0 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08A5.99 5.99 0 0 1 12 18c-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/>
        </svg>
        Reset
      </button>
    </div>

    <div class="section-content">
      <!-- Enable Toggle -->
      <div class="setting-item toggle-setting">
        <div class="setting-info">
          <span class="setting-label">Enable Alerts</span>
          <span class="setting-desc">Receive notifications when heart rate exceeds thresholds</span>
        </div>
        <label class="toggle-switch">
          <input type="checkbox" bind:checked={localSettings.enabled} />
          <span class="toggle-slider"></span>
        </label>
      </div>

      <!-- Thresholds -->
      <div class="thresholds-grid" class:disabled={!localSettings.enabled}>
        <div class="threshold-card low">
          <div class="threshold-header">
            <svg viewBox="0 0 24 24" fill="currentColor" width="20" height="20">
              <path d="M16 18v2H8v-2h8zM11 7.99v8h2v-8h3l-4-4-4 4h3z"/>
            </svg>
            <span>Low Threshold</span>
          </div>
          <div class="threshold-input">
            <input
              type="number"
              bind:value={localSettings.low_threshold}
              min="30"
              max="100"
              disabled={!localSettings.enabled}
            />
            <span class="unit">BPM</span>
          </div>
          <label class="checkbox-row">
            <input
              type="checkbox"
              bind:checked={localSettings.notify_on_low}
              disabled={!localSettings.enabled}
            />
            <span>Notify when below</span>
          </label>
        </div>

        <div class="threshold-card high">
          <div class="threshold-header">
            <svg viewBox="0 0 24 24" fill="currentColor" width="20" height="20">
              <path d="M16 18v2H8v-2h8zM11 14.01V6.01h2v8h-2zm0 4v-2h2v2h-2z"/>
            </svg>
            <span>High Threshold</span>
          </div>
          <div class="threshold-input">
            <input
              type="number"
              bind:value={localSettings.high_threshold}
              min="100"
              max="220"
              disabled={!localSettings.enabled}
            />
            <span class="unit">BPM</span>
          </div>
          <label class="checkbox-row">
            <input
              type="checkbox"
              bind:checked={localSettings.notify_on_high}
              disabled={!localSettings.enabled}
            />
            <span>Notify when above</span>
          </label>
        </div>
      </div>

      <!-- Save Button -->
      {#if settingsUpdated}
        <button class="btn-save" on:click={saveSettings}>
          <svg viewBox="0 0 24 24" fill="currentColor" width="16" height="16">
            <path d="M17 3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V7l-4-4zm-5 16c-1.66 0-3-1.34-3-3s1.34-3 3-3 3 1.34 3 3-1.34 3-3 3zm3-10H5V5h10v4z"/>
          </svg>
          Save Changes
        </button>
      {/if}
    </div>
  </div>

  <!-- Exercise Types -->
  <div class="settings-section glass-card">
    <div class="section-header">
      <div class="header-left">
        <svg viewBox="0 0 24 24" fill="currentColor" width="24" height="24">
          <path d="M13.49 5.48c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm-3.6 13.9l1-4.4 2.1 2v6h2v-7.5l-2.1-2 .6-3c1.3 1.5 3.3 2.5 5.5 2.5v-2c-1.9 0-3.5-1-4.3-2.4l-1-1.6c-.4-.6-1-1-1.7-1-.3 0-.5.1-.8.1l-5.2 2.2v4.7h2v-3.4l1.8-.7-1.6 8.1-4.9-1-.4 2 7 1.4z"/>
        </svg>
        <h3>Exercise Types</h3>
      </div>
    </div>
    <ExerciseTypesEditor />
  </div>
</div>

<style>
  .settings-container {
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .settings-section {
    padding: 24px;
  }

  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;
    padding-bottom: 16px;
    border-bottom: 1px solid var(--border-color);
  }

  .header-left {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .header-left svg {
    color: var(--primary-color);
  }

  .settings-section:last-child .header-left svg {
    color: var(--warning-color);
  }

  h3 {
    font-size: 18px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .btn-reset {
    display: flex;
    align-items: center;
    gap: 6px;
    background: transparent;
    color: var(--text-secondary);
    font-size: 13px;
    padding: 8px 12px;
    border: 1px solid var(--border-color);
    border-radius: 8px;
    transition: all 0.2s ease;
  }

  .btn-reset:hover {
    border-color: var(--text-secondary);
    color: var(--text-primary);
  }

  .fullscreen-options {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .bpm-note {
    margin: 8px 0 0;
    font-size: 12px;
    color: var(--text-muted);
    text-align: center;
    font-style: italic;
  }

  .section-content {
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .toggle-setting {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px;
    background: var(--bg-color);
    border-radius: 12px;
  }

  .setting-info {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .setting-label {
    font-size: 14px;
    font-weight: 500;
    color: var(--text-primary);
  }

  .setting-desc {
    font-size: 12px;
    color: var(--text-muted);
  }

  .toggle-switch {
    position: relative;
    width: 48px;
    height: 26px;
    cursor: pointer;
  }

  .toggle-switch input {
    opacity: 0;
    width: 0;
    height: 0;
  }

  .toggle-slider {
    position: absolute;
    inset: 0;
    background: var(--border-color);
    border-radius: 13px;
    transition: all 0.3s ease;
  }

  .toggle-slider::before {
    content: '';
    position: absolute;
    width: 20px;
    height: 20px;
    left: 3px;
    bottom: 3px;
    background: white;
    border-radius: 50%;
    transition: all 0.3s ease;
  }

  .toggle-switch input:checked + .toggle-slider {
    background: var(--primary-color);
  }

  .toggle-switch input:checked + .toggle-slider::before {
    transform: translateX(22px);
  }

  .thresholds-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
    transition: opacity 0.3s ease;
  }

  .thresholds-grid.disabled {
    opacity: 0.5;
    pointer-events: none;
  }

  .threshold-card {
    padding: 16px;
    background: var(--bg-color);
    border-radius: 12px;
    border: 1px solid var(--border-color);
  }

  .threshold-card.low {
    border-left: 3px solid var(--primary-color);
  }

  .threshold-card.high {
    border-left: 3px solid var(--danger-color);
  }

  .threshold-header {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 12px;
    font-size: 13px;
    font-weight: 500;
    color: var(--text-secondary);
  }

  .threshold-card.low .threshold-header svg {
    color: var(--primary-color);
  }

  .threshold-card.high .threshold-header svg {
    color: var(--danger-color);
  }

  .threshold-input {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 12px;
  }

  .threshold-input input {
    flex: 1;
    background: var(--card-bg);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    padding: 10px 12px;
    color: var(--text-primary);
    font-size: 18px;
    font-weight: 600;
    text-align: center;
    font-variant-numeric: tabular-nums;
  }

  .threshold-input input:focus {
    outline: none;
    border-color: var(--primary-color);
  }

  .threshold-input .unit {
    font-size: 13px;
    color: var(--text-muted);
  }

  .checkbox-row {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 12px;
    color: var(--text-secondary);
    cursor: pointer;
  }

  .checkbox-row input {
    width: 14px;
    height: 14px;
    accent-color: var(--primary-color);
  }

  .btn-save {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    width: 100%;
    background: var(--primary-color);
    color: white;
    padding: 12px;
    border-radius: 10px;
    font-size: 14px;
    font-weight: 500;
    transition: all 0.2s ease;
  }

  .btn-save:hover {
    background: var(--primary-hover);
    box-shadow: var(--glow-primary);
  }

  @media (max-width: 500px) {
    .thresholds-grid {
      grid-template-columns: 1fr;
    }
  }
</style>