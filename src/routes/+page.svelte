<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';

  import HeartRateDisplay from '$lib/components/HeartRateDisplay.svelte';
  import HeartRateChart from '$lib/components/HeartRateChart.svelte';
  import DeviceScanner from '$lib/components/DeviceScanner.svelte';
  import AlertSettings from '$lib/components/AlertSettings.svelte';
  import HistoryView from '$lib/components/HistoryView.svelte';
  import ConnectionStatus from '$lib/components/ConnectionStatus.svelte';
  import FullscreenMode from '$lib/components/FullscreenMode.svelte';

  import { heartRate } from '$lib/stores/heartRate';
  import { device } from '$lib/stores/device';
  import { settings } from '$lib/stores/settings';
  import { history } from '$lib/stores/history';

  let activeTab = 'monitor';
  let theme = 'dark';
  let showScanPanel = false;
  let showFullscreen = false;

  // Theme toggle
  function toggleTheme() {
    theme = theme === 'dark' ? 'light' : 'dark';
    document.documentElement.setAttribute('data-theme', theme);
  }

  $: isConnected = $device.connectionState === 'Connected';

  // Close scan panel when connected
  $: if (isConnected && showScanPanel) {
    showScanPanel = false;
  }

  onMount(async () => {
    // Initialize Tauri event listeners and load settings
    await Promise.all([
      heartRate.initListener(),
      device.initListeners(),
      settings.loadSettings(),
    ]);

    // Sync connection state from backend
    await device.syncConnectionState();

    // Initialize database
    try {
      await invoke('init_database');
    } catch (error) {
      console.error('Failed to initialize database:', error);
    }
  });

  // Cleanup on destroy
  onMount(() => {
    return () => {
      heartRate.cleanup();
      device.cleanup();
    };
  });
</script>

<svelte:head>
  <title>Health Tracker - Heart Rate Monitor</title>
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous">
  <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap" rel="stylesheet">
</svelte:head>

<div class="app">
  <header class="app-header">
    <div class="logo">
      <div class="heart-icon">
        <svg viewBox="0 0 24 24" fill="currentColor" width="28" height="28">
          <path d="M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z"/>
        </svg>
      </div>
      <h1>Health Tracker</h1>
    </div>
    <div class="header-right">
      {#if isConnected}
        <button class="fullscreen-btn" on:click={() => showFullscreen = true} title="Fullscreen">
          <svg viewBox="0 0 24 24" fill="currentColor" width="18" height="18">
            <path d="M7 14H5v5h5v-2H7v-3zm-2-4h2V7h3V5H5v5zm12 7h-3v2h5v-5h-2v3zM14 5v2h3v3h2V5h-5z"/>
          </svg>
        </button>
      {/if}
      {#if !isConnected}
        <button class="scan-btn" on:click={() => showScanPanel = true}>
          <svg viewBox="0 0 24 24" fill="currentColor" width="18" height="18">
            <path d="M15.5 14h-.79l-.28-.27A6.471 6.471 0 0 0 16 9.5 6.5 6.5 0 1 0 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z"/>
          </svg>
          Scan
        </button>
      {/if}
      <button class="theme-toggle" on:click={toggleTheme} title="Toggle theme">
        {#if theme === 'dark'}
          <svg viewBox="0 0 24 24" fill="currentColor" width="20" height="20">
            <path d="M12 3a9 9 0 1 0 9 9c0-.46-.04-.92-.1-1.36a5.389 5.389 0 0 1-4.4 2.26 5.403 5.403 0 0 1-3.14-9.8c-.44-.06-.9-.1-1.36-.1z"/>
          </svg>
        {:else}
          <svg viewBox="0 0 24 24" fill="currentColor" width="20" height="20">
            <path d="M12 7c-2.76 0-5 2.24-5 5s2.24 5 5 5 5-2.24 5-5-2.24-5-5-5zM2 13h2c.55 0 1-.45 1-1s-.45-1-1-1H2c-.55 0-1 .45-1 1s.45 1 1 1zm18 0h2c.55 0 1-.45 1-1s-.45-1-1-1h-2c-.55 0-1 .45-1 1s.45 1 1 1zM11 2v2c0 .55.45 1 1 1s1-.45 1-1V2c0-.55-.45-1-1-1s-1 .45-1 1zm0 18v2c0 .55.45 1 1 1s1-.45 1-1v-2c0-.55-.45-1-1-1s-1 .45-1 1zM5.99 4.58a.996.996 0 0 0-1.41 0 .996.996 0 0 0 0 1.41l1.06 1.06c.39.39 1.03.39 1.41 0s.39-1.03 0-1.41L5.99 4.58zm12.37 12.37a.996.996 0 0 0-1.41 0 .996.996 0 0 0 0 1.41l1.06 1.06c.39.39 1.03.39 1.41 0a.996.996 0 0 0 0-1.41l-1.06-1.06zm1.06-10.96a.996.996 0 0 0 0-1.41.996.996 0 0 0-1.41 0l-1.06 1.06c-.39.39-.39 1.03 0 1.41s1.03.39 1.41 0l1.06-1.06zM7.05 18.36a.996.996 0 0 0 0-1.41.996.996 0 0 0-1.41 0l-1.06 1.06c-.39.39-.39 1.03 0 1.41s1.03.39 1.41 0l1.06-1.06z"/>
          </svg>
        {/if}
      </button>
      <ConnectionStatus />
    </div>
  </header>

  <nav class="tabs">
    <button
      class="tab"
      class:active={activeTab === 'monitor'}
      on:click={() => activeTab = 'monitor'}
    >
      <svg viewBox="0 0 24 24" fill="currentColor" width="18" height="18">
        <path d="M3 13h2v8H3v-8zm4-6h2v14H7V7zm4-4h2v18h-2V3zm4 8h2v10h-2V11zm4-3h2v13h-2V8z"/>
      </svg>
      Monitor
    </button>
    <button
      class="tab"
      class:active={activeTab === 'history'}
      on:click={() => activeTab = 'history'}
    >
      <svg viewBox="0 0 24 24" fill="currentColor" width="18" height="18">
        <path d="M13 3a9 9 0 0 0-9 9H1l3.89 3.89.07.14L9 12H6c0-3.87 3.13-7 7-7s7 3.13 7 7-3.13 7-7 7c-1.93 0-3.68-.79-4.94-2.06l-1.42 1.42A8.954 8.954 0 0 0 13 21a9 9 0 0 0 0-18zm-1 5v5l4.28 2.54.72-1.21-3.5-2.08V8H12z"/>
      </svg>
      History
    </button>
    <button
      class="tab"
      class:active={activeTab === 'settings'}
      on:click={() => activeTab = 'settings'}
    >
      <svg viewBox="0 0 24 24" fill="currentColor" width="18" height="18">
        <path d="M19.14 12.94c.04-.31.06-.63.06-.94 0-.31-.02-.63-.06-.94l2.03-1.58a.49.49 0 0 0 .12-.61l-1.92-3.32a.488.488 0 0 0-.59-.22l-2.39.96c-.5-.38-1.03-.7-1.62-.94l-.36-2.54a.484.484 0 0 0-.48-.41h-3.84c-.24 0-.43.17-.47.41l-.36 2.54c-.59.24-1.13.57-1.62.94l-2.39-.96c-.22-.08-.47 0-.59.22L2.74 8.87c-.12.21-.08.47.12.61l2.03 1.58c-.04.31-.06.63-.06.94s.02.63.06.94l-2.03 1.58a.49.49 0 0 0-.12.61l1.92 3.32c.12.22.37.29.59.22l2.39-.96c.5.38 1.03.7 1.62.94l.36 2.54c.05.24.24.41.48.41h3.84c.24 0 .44-.17.47-.41l.36-2.54c.59-.24 1.13-.56 1.62-.94l2.39.96c.22.08.47 0 .59-.22l1.92-3.32c.12-.22.07-.47-.12-.61l-2.01-1.58zM12 15.6c-1.98 0-3.6-1.62-3.6-3.6s1.62-3.6 3.6-3.6 3.6 1.62 3.6 3.6-1.62 3.6-3.6 3.6z"/>
      </svg>
      Settings
    </button>
  </nav>

  <main class="content">
    {#if activeTab === 'monitor'}
      <div class="monitor-view">
        <HeartRateDisplay />
        <HeartRateChart />
      </div>
    {:else if activeTab === 'history'}
      <div class="single-column">
        <HistoryView />
      </div>
    {:else if activeTab === 'settings'}
      <div class="settings-column">
        <AlertSettings />
      </div>
    {/if}
  </main>

  <!-- Slide-out Scan Panel -->
  {#if showScanPanel}
    <div class="overlay" on:click={() => showScanPanel = false}></div>
    <div class="scan-panel">
      <div class="panel-header">
        <h3>Scan Devices</h3>
        <button class="close-btn" on:click={() => showScanPanel = false}>
          <svg viewBox="0 0 24 24" fill="currentColor" width="20" height="20">
            <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/>
          </svg>
        </button>
      </div>
      <DeviceScanner />
    </div>
  {/if}

  <!-- Fullscreen Mode -->
  {#if showFullscreen}
    <FullscreenMode onClose={() => showFullscreen = false} />
  {/if}
</div>

<style>
  .app {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    padding: 20px;
    max-width: 1400px;
    margin: 0 auto;
  }

  .app-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;
    padding-bottom: 16px;
    border-bottom: 1px solid var(--border-color);
  }

  .logo {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .heart-icon {
    color: var(--danger-color);
    animation: heartbeat 1.5s ease-in-out infinite;
  }

  @keyframes heartbeat {
    0%, 100% { transform: scale(1); }
    14% { transform: scale(1.15); }
    28% { transform: scale(1); }
    42% { transform: scale(1.15); }
    70% { transform: scale(1); }
  }

  h1 {
    font-size: 22px;
    font-weight: 700;
    background: linear-gradient(135deg, var(--text-primary) 0%, var(--text-secondary) 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .header-right {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .scan-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    background: var(--primary-color);
    color: white;
    padding: 8px 14px;
    border-radius: 10px;
    font-size: 13px;
    font-weight: 500;
    transition: all 0.2s ease;
  }

  .scan-btn:hover {
    background: var(--primary-hover);
    box-shadow: var(--glow-primary);
  }

  .fullscreen-btn {
    width: 40px;
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--card-bg);
    border: 1px solid var(--border-color);
    border-radius: 10px;
    color: var(--text-secondary);
    transition: all 0.2s ease;
    padding: 0;
  }

  .fullscreen-btn:hover {
    color: var(--text-primary);
    background: var(--card-bg-hover);
    border-color: var(--primary-color);
  }

  .theme-toggle {
    width: 40px;
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--card-bg);
    border: 1px solid var(--border-color);
    border-radius: 10px;
    color: var(--text-secondary);
    transition: all 0.2s ease;
    padding: 0;
  }

  .theme-toggle:hover {
    color: var(--text-primary);
    background: var(--card-bg-hover);
    border-color: var(--primary-color);
  }

  .tabs {
    display: flex;
    gap: 8px;
    margin-bottom: 20px;
    flex-wrap: wrap;
  }

  .tab {
    display: flex;
    align-items: center;
    gap: 8px;
    background: var(--card-bg);
    color: var(--text-secondary);
    padding: 12px 20px;
    border-radius: 12px;
    font-size: 14px;
    font-weight: 500;
    border: 1px solid transparent;
    transition: all 0.2s ease;
  }

  .tab svg {
    opacity: 0.7;
  }

  .tab:hover {
    color: var(--text-primary);
    background: var(--card-bg-hover);
  }

  .tab:hover svg {
    opacity: 1;
  }

  .tab.active {
    color: var(--primary-color);
    background: var(--primary-light);
    border-color: var(--primary-color);
  }

  .tab.active svg {
    opacity: 1;
  }

  .content {
    flex: 1;
    animation: fadeIn 0.3s ease-out;
  }

  .monitor-view {
    display: flex;
    flex-direction: column;
    gap: 20px;
    max-width: 700px;
    margin: 0 auto;
  }

  .single-column {
    max-width: 600px;
    animation: slideIn 0.3s ease-out;
  }

  .settings-column {
    max-width: 600px;
    animation: slideIn 0.3s ease-out;
  }

  @media (max-width: 600px) {
    .app {
      padding: 12px;
    }

    .app-header {
      flex-direction: column;
      gap: 12px;
      align-items: flex-start;
    }

    .header-right {
      width: 100%;
      justify-content: space-between;
    }

    .tab {
      padding: 10px 14px;
      font-size: 13px;
    }

    .tab span {
      display: none;
    }

    .tab svg {
      opacity: 1;
    }
  }

  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    backdrop-filter: blur(4px);
    z-index: 100;
    animation: fadeIn 0.2s ease;
  }

  .scan-panel {
    position: fixed;
    top: 0;
    right: 0;
    bottom: 0;
    width: 360px;
    max-width: 90vw;
    background: var(--card-bg);
    border-left: 1px solid var(--border-color);
    box-shadow: -4px 0 24px var(--shadow-color);
    z-index: 101;
    animation: slideIn 0.3s ease;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
  }

  .panel-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 20px;
    border-bottom: 1px solid var(--border-color);
  }

  .panel-header h3 {
    font-size: 18px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .close-btn {
    width: 36px;
    height: 36px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    color: var(--text-secondary);
    border-radius: 8px;
    transition: all 0.2s ease;
  }

  .close-btn:hover {
    background: var(--bg-color);
    color: var(--text-primary);
  }

  .scan-panel :global(.scanner-container) {
    border-radius: 0;
    border: none;
    border-bottom: 1px solid var(--border-color);
    flex: 1;
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  @keyframes slideIn {
    from { transform: translateX(100%); }
    to { transform: translateX(0); }
  }
</style>