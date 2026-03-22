<script lang="ts">
  import { disconnectDevice } from '$lib/utils/tauri';
  import { device } from '$lib/stores/device';

  $: state = $device.connectionState;
  $: connectedDevice = $device.connectedDevice;

  async function handleDisconnect() {
    try {
      await disconnectDevice();
    } catch (error) {
      console.error('Failed to disconnect:', error);
    }
  }

  function getStateDisplay(): { text: string; color: string; animate: boolean } {
    if (state === 'Connected') {
      return { text: 'Connected', color: 'var(--success-color)', animate: false };
    }
    if (state === 'Connecting') {
      return { text: 'Connecting...', color: 'var(--warning-color)', animate: true };
    }
    if (state === 'Disconnecting') {
      return { text: 'Disconnecting...', color: 'var(--warning-color)', animate: true };
    }
    if (typeof state === 'object' && 'Error' in state) {
      return { text: 'Error', color: 'var(--danger-color)', animate: false };
    }
    return { text: 'Disconnected', color: 'var(--text-muted)', animate: false };
  }

  $: display = getStateDisplay();
</script>

{#if state === 'Connected' && connectedDevice}
  <div class="connected-info" title="{connectedDevice.name}">
    <span class="device-name">{connectedDevice.name}</span>
    <button class="btn-disconnect" on:click={handleDisconnect} title="Disconnect">
      <svg viewBox="0 0 24 24" fill="currentColor" width="14" height="14">
        <path d="M19.61 12.91L15.7 8.99l1.42-1.42 4.91 4.92-4.92 4.92-1.41-1.42 3.91-3.91zM5.39 12.91l3.91 3.91-1.42 1.42-4.91-4.92 4.92-4.92 1.41 1.42-3.91 3.91z"/>
      </svg>
    </button>
  </div>
{:else}
  <div class="connection-status" style="--status-color: {display.color}">
    <span class="status-dot" class:animate={display.animate}></span>
    <span class="status-text">{display.text}</span>
  </div>
{/if}

<style>
  .connected-info {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 6px 10px 6px 14px;
    background: var(--success-light);
    border: 1px solid color-mix(in srgb, var(--success-color) 30%, transparent);
    border-radius: 24px;
  }

  .device-name {
    font-size: 13px;
    font-weight: 500;
    color: var(--success-color);
    white-space: nowrap;
  }

  .btn-disconnect {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    background: var(--danger-color);
    color: white;
    border-radius: 50%;
    transition: all 0.2s ease;
    padding: 0;
  }

  .btn-disconnect:hover {
    background: #dc2626;
    box-shadow: var(--glow-danger);
    transform: scale(1.05);
  }

  .connection-status {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 8px 14px;
    border-radius: 24px;
    font-size: 13px;
    font-weight: 500;
    background: color-mix(in srgb, var(--status-color) 15%, transparent);
    border: 1px solid color-mix(in srgb, var(--status-color) 30%, transparent);
    transition: all 0.3s ease;
  }

  .status-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--status-color);
    box-shadow: 0 0 8px var(--status-color);
  }

  .status-dot.animate {
    animation: pulse-dot 1.5s ease-in-out infinite;
  }

  @keyframes pulse-dot {
    0%, 100% { opacity: 1; transform: scale(1); }
    50% { opacity: 0.5; transform: scale(0.8); }
  }

  .status-text {
    color: var(--status-color);
  }
</style>