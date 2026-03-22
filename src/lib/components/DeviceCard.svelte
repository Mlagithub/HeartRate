<script lang="ts">
  import type { DeviceInfo } from '$lib/stores/device';

  export let dev: DeviceInfo;
  export let isConnected: boolean;
  export let onConnect: () => void;

  function getSignalStrength(rssi: number): { bars: number; color: string; label: string } {
    if (rssi >= -50) return { bars: 4, color: 'var(--success-color)', label: 'Excellent' };
    if (rssi >= -60) return { bars: 3, color: '#84cc16', label: 'Good' };
    if (rssi >= -75) return { bars: 2, color: 'var(--warning-color)', label: 'Fair' };
    return { bars: 1, color: 'var(--danger-color)', label: 'Weak' };
  }

  $: signal = getSignalStrength(dev.rssi);
</script>

<div class="device-card" class:connected={isConnected} class:hr-device={dev.supports_heart_rate}>
  <div class="device-icon" style="background: {dev.supports_heart_rate ? 'var(--primary-light)' : 'var(--border-color)'}">
    <svg viewBox="0 0 24 24" fill="currentColor" width="20" height="20">
      <path d="M17 1.01L7 1c-1.1 0-2 .9-2 2v18c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V3c0-1.1-.9-1.99-2-1.99zM17 19H7V5h10v14z"/>
    </svg>
  </div>

  <div class="device-info">
    <div class="device-name">
      {dev.name || 'Unknown Device'}
      {#if dev.supports_heart_rate}
        <span class="badge hr">HR</span>
      {/if}
    </div>
    <div class="device-meta">
      <div class="signal-bars">
        {#each Array(4) as _, i}
          <div
            class="bar"
            class:active={i < signal.bars}
            style="--bar-color: {signal.color}"
          ></div>
        {/each}
      </div>
      <span class="signal-text">{signal.label}</span>
      <span class="rssi">{dev.rssi} dBm</span>
    </div>
  </div>

  <div class="device-actions">
    {#if isConnected}
      <div class="connected-badge">
        <svg viewBox="0 0 24 24" fill="currentColor" width="14" height="14">
          <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z"/>
        </svg>
        Connected
      </div>
    {:else}
      <button
        class="btn-connect"
        on:click={onConnect}
        disabled={!dev.supports_heart_rate}
      >
        {#if dev.supports_heart_rate}
          Connect
        {:else}
          No HR
        {/if}
      </button>
    {/if}
  </div>
</div>

<style>
  .device-card {
    display: flex;
    align-items: center;
    gap: 14px;
    padding: 14px;
    background: var(--bg-color);
    border-radius: 12px;
    border: 1px solid var(--border-color);
    transition: all 0.2s ease;
  }

  .device-card:hover {
    border-color: var(--primary-color);
    transform: translateY(-1px);
  }

  .device-card.hr-device {
    background: linear-gradient(135deg, var(--bg-color) 0%, color-mix(in srgb, var(--primary-color) 5%, var(--bg-color)) 100%);
  }

  .device-card.connected {
    border-color: var(--success-color);
    background: var(--success-light);
  }

  .device-icon {
    width: 40px;
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 10px;
    color: var(--primary-color);
    flex-shrink: 0;
  }

  .device-info {
    flex: 1;
    min-width: 0;
  }

  .device-name {
    font-size: 14px;
    font-weight: 600;
    color: var(--text-primary);
    display: flex;
    align-items: center;
    gap: 8px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .badge {
    font-size: 9px;
    padding: 2px 6px;
    border-radius: 4px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    flex-shrink: 0;
  }

  .badge.hr {
    background: var(--primary-color);
    color: white;
  }

  .device-meta {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-top: 4px;
    font-size: 11px;
    color: var(--text-muted);
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
    background: var(--bar-color);
  }

  .signal-text {
    color: var(--text-secondary);
  }

  .rssi {
    font-variant-numeric: tabular-nums;
  }

  .device-actions {
    flex-shrink: 0;
  }

  .btn-connect {
    background: var(--primary-color);
    color: white;
    padding: 8px 16px;
    border-radius: 8px;
    font-size: 13px;
    font-weight: 500;
    transition: all 0.2s ease;
  }

  .btn-connect:hover:not(:disabled) {
    background: var(--primary-hover);
    box-shadow: var(--glow-primary);
  }

  .btn-connect:disabled {
    background: var(--border-color);
    color: var(--text-muted);
    cursor: not-allowed;
  }

  .connected-badge {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 8px 14px;
    background: var(--success-light);
    color: var(--success-color);
    border-radius: 8px;
    font-size: 13px;
    font-weight: 600;
  }
</style>