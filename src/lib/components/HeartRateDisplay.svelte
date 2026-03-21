<script lang="ts">
  import { heartRate, type HeartRateStats } from '$lib/stores/heartRate';
  import { device } from '$lib/stores/device';

  $: currentBpm = $heartRate.current?.bpm ?? null;
  $: isConnected = $device.connectionState === 'Connected';
  $: stats = $heartRate.stats;

  // Heart rate zones based on percentage of max (assuming max 220 - age, using 180 as default)
  const zones = [
    { name: 'Rest', min: 0, max: 60, color: '#94a3b8', bg: 'rgba(148, 163, 184, 0.15)' },
    { name: 'Fat Burn', min: 60, max: 100, color: '#22c55e', bg: 'rgba(34, 197, 94, 0.15)' },
    { name: 'Cardio', min: 100, max: 140, color: '#f59e0b', bg: 'rgba(245, 158, 11, 0.15)' },
    { name: 'Peak', min: 140, max: 170, color: '#ef4444', bg: 'rgba(239, 68, 68, 0.15)' },
    { name: 'Extreme', min: 170, max: 220, color: '#dc2626', bg: 'rgba(220, 38, 38, 0.15)' },
  ];

  $: currentZone = currentBpm
    ? zones.find(z => currentBpm >= z.min && currentBpm < z.max) || zones[zones.length - 1]
    : null;

  $: bpmColor = currentBpm ? currentZone?.color ?? '#94a3b8' : '#64748b';

  function formatTime(ts: number): string {
    return new Date(ts).toLocaleTimeString('en-US', {
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
    });
  }

  function getGlowColor(zone: typeof zones[0] | null): string {
    if (!zone) return '';
    return `0 0 40px ${zone.bg}, 0 0 80px ${zone.bg}`;
  }
</script>

<div class="heart-rate-display glass-card">
  <!-- Main BPM Display -->
  <div class="bpm-section" style="--glow: {getGlowColor(currentZone)}">
    <div class="bpm-ring" style="--zone-color: {bpmColor}">
      <div class="bpm-value" style="color: {bpmColor}">
        {currentBpm ?? '--'}
      </div>
      <div class="bpm-unit">BPM</div>
    </div>

    <!-- Zone Indicator -->
    {#if currentZone && currentBpm}
      <div class="zone-badge" style="background: {currentZone.bg}; color: {currentZone.color}">
        {currentZone.name}
      </div>
    {/if}
  </div>

  <!-- Zone Bar -->
  <div class="zone-bar">
    {#each zones as zone, i}
      <div
        class="zone-segment"
        class:active={currentBpm && currentBpm >= zone.min && currentBpm < zone.max}
        style="background: {zone.color}; --zone-bg: {zone.bg}"
        title="{zone.name}: {zone.min}-{zone.max} BPM"
      >
        <span class="zone-label">{zone.min}</span>
      </div>
    {/each}
  </div>

  <!-- Stats Grid -->
  {#if stats.count > 0}
    <div class="stats-grid">
      <div class="stat-item">
        <span class="stat-label">Min</span>
        <span class="stat-value min">{stats.min}</span>
      </div>
      <div class="stat-item">
        <span class="stat-label">Avg</span>
        <span class="stat-value avg">{stats.avg}</span>
      </div>
      <div class="stat-item">
        <span class="stat-label">Max</span>
        <span class="stat-value max">{stats.max}</span>
      </div>
      <div class="stat-item">
        <span class="stat-label">Count</span>
        <span class="stat-value">{stats.count}</span>
      </div>
    </div>
  {/if}

  <!-- Status Bar -->
  <div class="status-bar">
    <div class="connection-status">
      {#if isConnected}
        <span class="status-dot connected"></span>
        <span>Connected</span>
      {:else}
        <span class="status-dot disconnected"></span>
        <span>Disconnected</span>
      {/if}
    </div>
    {#if currentBpm}
      <div class="last-update">
        {formatTime($heartRate.current?.timestamp ?? Date.now())}
      </div>
    {/if}
  </div>
</div>

<style>
  .heart-rate-display {
    padding: 24px;
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .bpm-section {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 16px;
  }

  .bpm-ring {
    width: 180px;
    height: 180px;
    border-radius: 50%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    background: radial-gradient(circle, var(--card-bg) 0%, transparent 70%);
    border: 4px solid var(--zone-color, var(--primary-color));
    box-shadow: var(--glow), inset 0 0 30px rgba(0, 0, 0, 0.3);
    transition: all 0.5s ease;
  }

  .bpm-value {
    font-size: 64px;
    font-weight: 700;
    line-height: 1;
    font-variant-numeric: tabular-nums;
    transition: color 0.3s ease;
  }

  .bpm-unit {
    font-size: 18px;
    color: var(--text-secondary);
    font-weight: 500;
    margin-top: 4px;
  }

  .zone-badge {
    padding: 6px 16px;
    border-radius: 20px;
    font-size: 14px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    transition: all 0.3s ease;
  }

  .zone-bar {
    display: flex;
    height: 8px;
    border-radius: 4px;
    overflow: hidden;
    gap: 2px;
  }

  .zone-segment {
    flex: 1;
    border-radius: 2px;
    position: relative;
    opacity: 0.4;
    transition: all 0.3s ease;
    cursor: pointer;
  }

  .zone-segment:hover {
    opacity: 0.7;
  }

  .zone-segment.active {
    opacity: 1;
    box-shadow: 0 0 12px var(--zone-bg);
  }

  .zone-label {
    position: absolute;
    bottom: -18px;
    left: 0;
    font-size: 10px;
    color: var(--text-muted);
    white-space: nowrap;
  }

  .zone-segment:last-child .zone-label {
    left: auto;
    right: 0;
  }

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 12px;
    padding-top: 8px;
    border-top: 1px solid var(--border-color);
  }

  .stat-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
  }

  .stat-label {
    font-size: 11px;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.5px;
    font-weight: 600;
    margin-bottom: 4px;
  }

  .stat-value {
    font-size: 20px;
    font-weight: 600;
    font-variant-numeric: tabular-nums;
    color: var(--text-primary);
  }

  .stat-value.min { color: var(--zone-rest); }
  .stat-value.avg { color: var(--zone-fat-burn); }
  .stat-value.max { color: var(--zone-peak); }

  .status-bar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 12px;
    color: var(--text-secondary);
    padding-top: 12px;
    border-top: 1px solid var(--border-color);
  }

  .connection-status {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .status-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
  }

  .status-dot.connected {
    background: var(--success-color);
    box-shadow: 0 0 8px var(--success-color);
    animation: pulse 2s ease-in-out infinite;
  }

  .status-dot.disconnected {
    background: var(--text-muted);
  }

  .last-update {
    font-variant-numeric: tabular-nums;
  }

  @media (max-width: 600px) {
    .bpm-ring {
      width: 150px;
      height: 150px;
    }

    .bpm-value {
      font-size: 48px;
    }

    .stats-grid {
      grid-template-columns: repeat(2, 1fr);
    }
  }
</style>