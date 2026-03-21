<script lang="ts">
  import { heartRate } from '$lib/stores/heartRate';
  import { settings } from '$lib/stores/settings';
  import { onMount, onDestroy } from 'svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';

  export let onClose: () => void;

  $: currentBpm = $heartRate.current?.bpm ?? '--';
  $: history = $heartRate.history;
  $: stats = $heartRate.stats;
  $: prefs = $settings.fullscreenPreferences;

  function getZoneColor(bpm: number): string {
    if (bpm < 60) return '#94a3b8';
    if (bpm < 100) return '#22c55e';
    if (bpm < 140) return '#f59e0b';
    return '#ef4444';
  }

  $: zoneColor = typeof currentBpm === 'number' ? getZoneColor(currentBpm) : '#64748b';

  async function enterFullscreen() {
    try {
      const window = getCurrentWindow();
      await window.setFullscreen(true);
    } catch (e) {
      console.error('Failed to enter fullscreen:', e);
    }
  }

  async function exitFullscreen() {
    try {
      const window = getCurrentWindow();
      await window.setFullscreen(false);
    } catch (e) {
      console.error('Failed to exit fullscreen:', e);
    }
  }

  async function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      await exitFullscreen();
      onClose();
    }
  }

  onMount(async () => {
    document.addEventListener('keydown', handleKeydown);
    await enterFullscreen();
  });

  onDestroy(async () => {
    document.removeEventListener('keydown', handleKeydown);
    await exitFullscreen();
  });

  // Generate polyline points for the chart
  $: chartPoints = history.slice(-30).map((p, i, arr) => {
    const x = (i / Math.max(arr.length - 1, 1)) * 100;
    const y = 100 - ((p.bpm - 40) / 160) * 100;
    return `${x},${Math.max(5, Math.min(95, y))}`;
  }).join(' ');
</script>

<div class="fullscreen" style="--zone-color: {zoneColor}" on:click={async () => { await exitFullscreen(); onClose(); }}>
  <div class="content">
    <!-- BPM Display -->
    <div class="bpm" style="color: {zoneColor}">
      {currentBpm}
    </div>

    <div class="unit">BPM</div>

    <!-- Mini Chart (when showChart is enabled) -->
    {#if prefs.showChart && history.length > 1}
      <div class="chart">
        <svg viewBox="0 0 100 100" preserveAspectRatio="none">
          <polyline
            points={chartPoints}
            fill="none"
            stroke={zoneColor}
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
        </svg>
      </div>
    {/if}

    <!-- Stats (when showStats is enabled) -->
    {#if prefs.showStats && stats.count > 0}
      <div class="stats">
        <div class="stat">
          <span class="label">Min</span>
          <span class="value">{stats.min}</span>
        </div>
        <div class="stat">
          <span class="label">Avg</span>
          <span class="value">{stats.avg}</span>
        </div>
        <div class="stat">
          <span class="label">Max</span>
          <span class="value">{stats.max}</span>
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  .fullscreen {
    position: fixed;
    inset: 0;
    background: #000;
    z-index: 1000;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    user-select: none;
  }

  .content {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
  }

  .bpm {
    font-size: 220px;
    font-weight: 200;
    line-height: 1;
    font-variant-numeric: tabular-nums;
    text-shadow: 0 0 60px var(--zone-color);
    transition: color 0.5s ease, text-shadow 0.5s ease;
  }

  .unit {
    font-size: 28px;
    font-weight: 200;
    letter-spacing: 12px;
    color: rgba(255, 255, 255, 0.4);
    text-transform: uppercase;
  }

  .chart {
    width: 320px;
    height: 60px;
    margin-top: 24px;
    opacity: 0.5;
  }

  .chart svg {
    width: 100%;
    height: 100%;
  }

  .stats {
    display: flex;
    gap: 48px;
    margin-top: 24px;
  }

  .stat {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
  }

  .stat .label {
    font-size: 11px;
    color: rgba(255, 255, 255, 0.3);
    text-transform: uppercase;
    letter-spacing: 2px;
  }

  .stat .value {
    font-size: 24px;
    font-weight: 300;
    color: rgba(255, 255, 255, 0.6);
    font-variant-numeric: tabular-nums;
  }

  @media (max-width: 1200px) {
    .bpm {
      font-size: 160px;
    }
  }

  @media (max-width: 800px) {
    .bpm {
      font-size: 120px;
    }

    .unit {
      font-size: 20px;
      letter-spacing: 8px;
    }

    .chart {
      width: 240px;
      height: 50px;
    }

    .stats {
      gap: 32px;
    }

    .stat .value {
      font-size: 20px;
    }
  }
</style>