<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { device } from '$lib/stores/device';

  async function disconnect() {
    try {
      await invoke('disconnect_device');
    } catch (error) {
      console.error('Failed to disconnect:', error);
    }
  }

  $: isConnected = $device.connectionState === 'Connected';
  $: isConnecting = $device.connectionState === 'Connecting';
  $: isDisconnecting = $device.connectionState === 'Disconnecting';

  function getSignalStrength(rssi: number): { bars: number; label: string } {
    if (rssi >= -50) return { bars: 4, label: 'Excellent' };
    if (rssi >= -60) return { bars: 3, label: 'Good' };
    if (rssi >= -70) return { bars: 2, label: 'Fair' };
    return { bars: 1, label: 'Weak' };
  }

  $: signal = $device.connectedDevice ? getSignalStrength($device.connectedDevice.rssi) : null;
</script>

<div class="connection-panel glass-card">
  {#if isConnected || isConnecting}
    <div class="connected-device">
      <div class="device-icon">
        <svg viewBox="0 0 24 24" fill="currentColor" width="24" height="24">
          <path d="M17 1.01L7 1c-1.1 0-2 .9-2 2v18c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V3c0-1.1-.9-1.99-2-1.99zM17 19H7V5h10v14zm-4.2-5.78v1.75l3.2-2.99L12.8 9v1.7c-3.11.43-4.35 2.56-4.8 4.7 1.11-1.5 2.58-2.18 4.8-2.18z"/>
        </svg>
      </div>
      <div class="device-info">
        <span class="device-name">
          {$device.connectedDevice?.name || 'Unknown Device'}
        </span>
        {#if signal}
          <div class="signal-info">
            <div class="signal-bars">
              {#each Array(4) as _, i}
                <div class="bar {i < signal.bars ? 'active' : ''}"></div>
              {/each}
            </div>
            <span class="signal-label">{signal.label}</span>
          </div>
        {/if}
      </div>
      <button
        class="btn-disconnect"
        on:click={disconnect}
        disabled={isDisconnecting}
      >
        {#if isDisconnecting}
          <span class="spinner"></span>
          Disconnecting...
        {:else}
          <svg viewBox="0 0 24 24" fill="currentColor" width="16" height="16">
            <path d="M19.61 12.91L15.7 8.99l1.42-1.42 4.91 4.92-4.92 4.92-1.41-1.42 3.91-3.91zM5.39 12.91l3.91 3.91-1.42 1.42-4.91-4.92 4.92-4.92 1.41 1.42-3.91 3.91z"/>
          </svg>
          Disconnect
        {/if}
      </button>
    </div>
  {:else}
    <div class="no-connection">
      <div class="no-connection-icon">
        <svg viewBox="0 0 24 24" fill="currentColor" width="32" height="32">
          <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
        </svg>
      </div>
      <span class="message">No device connected</span>
      <span class="hint">Use the scanner above to find and connect to a heart rate monitor</span>
    </div>
  {/if}

  {#if typeof $device.connectionState === 'object' && 'Error' in $device.connectionState}
    <div class="error-message">
      <svg viewBox="0 0 24 24" fill="currentColor" width="16" height="16">
        <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-2h2v2zm0-4h-2V7h2v6z"/>
      </svg>
      {$device.connectionState.Error}
    </div>
  {/if}
</div>

<style>
  .connection-panel {
    padding: 20px;
  }

  .connected-device {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .device-icon {
    width: 48px;
    height: 48px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--primary-light);
    border-radius: 12px;
    color: var(--primary-color);
  }

  .device-info {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .device-name {
    font-size: 15px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .signal-info {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .signal-bars {
    display: flex;
    align-items: flex-end;
    gap: 2px;
    height: 12px;
  }

  .bar {
    width: 3px;
    background: var(--border-color);
    border-radius: 1px;
    transition: all 0.2s ease;
  }

  .bar:nth-child(1) { height: 3px; }
  .bar:nth-child(2) { height: 6px; }
  .bar:nth-child(3) { height: 9px; }
  .bar:nth-child(4) { height: 12px; }

  .bar.active {
    background: var(--success-color);
  }

  .signal-label {
    font-size: 12px;
    color: var(--text-muted);
  }

  .btn-disconnect {
    display: flex;
    align-items: center;
    gap: 6px;
    background: var(--danger-light);
    color: var(--danger-color);
    padding: 10px 16px;
    border-radius: 10px;
    font-size: 13px;
    font-weight: 500;
    border: 1px solid transparent;
    transition: all 0.2s ease;
  }

  .btn-disconnect:hover:not(:disabled) {
    background: var(--danger-color);
    color: white;
    box-shadow: var(--glow-danger);
  }

  .btn-disconnect:disabled {
    opacity: 0.7;
    cursor: not-allowed;
  }

  .spinner {
    width: 14px;
    height: 14px;
    border: 2px solid currentColor;
    border-top-color: transparent;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .no-connection {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    padding: 24px;
    gap: 10px;
  }

  .no-connection-icon {
    color: var(--text-muted);
    opacity: 0.5;
  }

  .message {
    font-size: 14px;
    font-weight: 500;
    color: var(--text-secondary);
  }

  .hint {
    font-size: 12px;
    color: var(--text-muted);
    max-width: 200px;
  }

  .error-message {
    margin-top: 16px;
    padding: 12px 14px;
    background: var(--danger-light);
    border: 1px solid color-mix(in srgb, var(--danger-color) 30%, transparent);
    border-radius: 10px;
    color: var(--danger-color);
    font-size: 13px;
    display: flex;
    align-items: center;
    gap: 10px;
  }

  @media (max-width: 600px) {
    .connected-device {
      flex-wrap: wrap;
    }

    .btn-disconnect {
      width: 100%;
      justify-content: center;
    }
  }
</style>