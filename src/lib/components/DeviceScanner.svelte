<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { device } from '$lib/stores/device';
  import DeviceCard from './DeviceCard.svelte';

  let scanError: string | null = null;

  // Ensure scanning is false on mount
  onMount(() => {
    device.setScanning(false);
  });

  async function startScan() {
    scanError = null;
    device.setScanning(true);
    device.clearDevices();

    try {
      await invoke('start_scan');

      // Auto-stop scan after 30 seconds
      setTimeout(() => {
        if ($device.isScanning) {
          stopScan();
        }
      }, 30000);
    } catch (error) {
      console.error('[DeviceScanner] start_scan error:', error);
      scanError = String(error);
      device.setScanning(false);
    }
  }

  async function stopScan() {
    try {
      await invoke('stop_scan');
    } catch (error) {
      console.error('[DeviceScanner] stop_scan error:', error);
    }
    device.setScanning(false);
  }

  async function handleScanClick() {
    if ($device.isScanning) {
      await stopScan();
    } else {
      await startScan();
    }
  }

  async function handleConnect(deviceId: string) {
    try {
      await invoke('connect_device', { deviceId });
    } catch (error) {
      console.error('[DeviceScanner] Failed to connect:', error);
      scanError = String(error);
    }
  }

  $: connectedId = $device.connectedDevice?.id;
  $: sortedDevices = [...$device.discoveredDevices].sort((a, b) => {
    // HR devices first
    if (a.supports_heart_rate && !b.supports_heart_rate) return -1;
    if (!a.supports_heart_rate && b.supports_heart_rate) return 1;
    // Then by signal strength
    return b.rssi - a.rssi;
  });
</script>

<div class="scanner-container glass-card">
  <div class="scanner-header">
    <h3>Device Scanner</h3>
    <button
      class="btn-scan"
      class:scanning={$device.isScanning}
      on:click={handleScanClick}
      disabled={$device.connectionState === 'Connected'}
    >
      {#if $device.isScanning}
        <span class="spinner"></span>
        Stop Scan
      {:else}
        <svg viewBox="0 0 24 24" fill="currentColor" width="16" height="16">
          <path d="M15.5 14h-.79l-.28-.27A6.471 6.471 0 0 0 16 9.5 6.5 6.5 0 1 0 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z"/>
        </svg>
        Scan
      {/if}
    </button>
  </div>

  {#if scanError}
    <div class="error-message">
      <svg viewBox="0 0 24 24" fill="currentColor" width="16" height="16">
        <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-2h2v2zm0-4h-2V7h2v6z"/>
      </svg>
      {scanError}
    </div>
  {/if}

  {#if $device.isScanning}
    <div class="scanning-indicator">
      <div class="pulse-ring">
        <div class="pulse-dot"></div>
      </div>
      <span>Scanning for devices...</span>
    </div>
  {/if}

  <div class="device-list">
    {#each sortedDevices as dev (dev.id)}
      <DeviceCard
        {dev}
        isConnected={dev.id === connectedId}
        onConnect={() => handleConnect(dev.id)}
      />
    {:else}
      {#if !$device.isScanning}
        <div class="no-devices">
          <svg viewBox="0 0 24 24" fill="currentColor" width="32" height="32">
            <path d="M17 1.01L7 1c-1.1 0-2 .9-2 2v18c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V3c0-1.1-.9-1.99-2-1.99zM17 19H7V5h10v14z"/>
          </svg>
          <span>Click Scan to discover nearby heart rate monitors</span>
        </div>
      {/if}
    {/each}
  </div>
</div>

<style>
  .scanner-container {
    padding: 20px;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .scanner-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  h3 {
    font-size: 16px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .btn-scan {
    display: flex;
    align-items: center;
    gap: 8px;
    background: var(--primary-color);
    color: white;
    padding: 10px 18px;
    border-radius: 10px;
    font-size: 14px;
    font-weight: 500;
    transition: all 0.2s ease;
  }

  .btn-scan:hover:not(:disabled) {
    background: var(--primary-hover);
    box-shadow: var(--glow-primary);
  }

  .btn-scan:disabled {
    background: var(--text-muted);
    cursor: not-allowed;
  }

  .btn-scan.scanning {
    background: var(--danger-color);
  }

  .btn-scan.scanning:hover {
    box-shadow: var(--glow-danger);
  }

  .spinner {
    width: 14px;
    height: 14px;
    border: 2px solid white;
    border-top-color: transparent;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .scanning-indicator {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 16px;
    color: var(--text-secondary);
    font-size: 14px;
    background: var(--bg-color);
    border-radius: 10px;
  }

  .pulse-ring {
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .pulse-dot {
    width: 12px;
    height: 12px;
    background: var(--primary-color);
    border-radius: 50%;
    animation: pulse 1.5s ease-in-out infinite;
  }

  @keyframes pulse {
    0%, 100% { transform: scale(1); opacity: 1; }
    50% { transform: scale(1.3); opacity: 0.7; }
  }

  .device-list {
    display: flex;
    flex-direction: column;
    gap: 10px;
    max-height: 280px;
    overflow-y: auto;
    padding-right: 4px;
  }

  .no-devices {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
    padding: 32px;
    color: var(--text-muted);
    font-size: 14px;
    text-align: center;
  }

  .no-devices svg {
    opacity: 0.5;
  }

  .error-message {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 12px 14px;
    background: var(--danger-light);
    border: 1px solid color-mix(in srgb, var(--danger-color) 30%, transparent);
    border-radius: 10px;
    color: var(--danger-color);
    font-size: 13px;
  }
</style>