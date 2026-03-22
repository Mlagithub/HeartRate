<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { statistics } from '$lib/stores/statistics';
  import StatisticsChart from './StatisticsChart.svelte';

  // Time dimension options
  const DIMENSIONS = ['daily', 'weekly', 'monthly', 'yearly'] as const;
  type Dimension = typeof DIMENSIONS[number];

  // HRV types (from 03-04 PLAN, D-09, D-10, D-11, D-12)
  interface HRVResult {
    hrv_value: number;
    is_estimated: boolean;
    confidence: string;
    data_points: number;
    period_start: number;
    period_end: number;
  }

  // Current period stats for summary cards (most recent period)
  $: latestStats = $statistics.stats.length > 0
    ? $statistics.stats[$statistics.stats.length - 1]
    : null;

  // Previous period for trend comparison (second to last)
  $: previousStats = $statistics.stats.length > 1
    ? $statistics.stats[$statistics.stats.length - 2]
    : null;

  // HRV state
  let hrvData: HRVResult | null = null;
  let hrvLoading = false;
  let hrvError: string | null = null;

  function selectDimension(dim: Dimension) {
    statistics.loadStatistics(dim);
  }

  function formatDimension(dim: string): string {
    return dim.charAt(0).toUpperCase() + dim.slice(1);
  }

  function formatPeriod(period: string, dimension: string): string {
    // Basic formatting based on dimension type
    if (dimension === 'daily') {
      return period; // YYYY-MM-DD
    } else if (dimension === 'weekly') {
      return period.replace('-', ' Week '); // YYYY-WW -> YYYY Week WW
    } else if (dimension === 'monthly') {
      return period; // YYYY-MM
    } else {
      return period; // YYYY
    }
  }

  function calculateTrend(current: number, previous: number): { value: number; direction: 'up' | 'down' | 'stable' } {
    if (previous === 0) return { value: 0, direction: 'stable' };
    const change = ((current - previous) / previous) * 100;
    return {
      value: Math.abs(change),
      direction: change > 1 ? 'up' : change < -1 ? 'down' : 'stable'
    };
  }

  async function loadHRV() {
    hrvLoading = true;
    hrvError = null;
    try {
      hrvData = await invoke<HRVResult | null>('get_hrv_estimate', {
        startTime: null,
        endTime: null
      });
    } catch (e) {
      hrvError = String(e);
    }
    hrvLoading = false;
  }

  onMount(() => {
    // Load with a small delay to avoid blocking
    setTimeout(() => {
      // Load default dimension (weekly per D-06)
      if ($statistics.stats.length === 0) {
        statistics.loadStatistics('weekly');
      }
      loadHRV();
    }, 150);
  });
</script>

<div class="statistics-tab">
  <!-- Time Dimension Selector (D-05, D-07) -->
  <div class="dimension-header">
    <h3>Heart Rate Statistics</h3>
    <div class="dimension-pills">
      {#each DIMENSIONS as dim}
        <button
          class="pill"
          class:active={$statistics.dimension === dim}
          on:click={() => selectDimension(dim)}
        >
          {formatDimension(dim)}
        </button>
      {/each}
    </div>
  </div>

  <!-- Loading State -->
  {#if $statistics.isLoading}
    <div class="loading-state">
      <div class="spinner"></div>
      <span>Loading statistics...</span>
    </div>

  <!-- Error State -->
  {:else if $statistics.error}
    <div class="error-state">
      <svg viewBox="0 0 24 24" fill="currentColor" width="20" height="20">
        <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-2h2v2zm0-4h-2V7h2v6z"/>
      </svg>
      <span>Error: {$statistics.error}</span>
    </div>

  <!-- Empty State -->
  {:else if $statistics.stats.length === 0}
    <div class="empty-state">
      <svg viewBox="0 0 24 24" fill="currentColor" width="48" height="48">
        <path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V5h14v14zM7 10h2v7H7zm4-3h2v10h-2zm4 6h2v4h-2z"/>
      </svg>
      <span>No statistics available</span>
      <span class="hint">Record some heart rate data to see statistics</span>
    </div>

  <!-- Statistics Content -->
  {:else}
    <!-- Summary Cards (D-02, D-09, D-11) -->
    <div class="summary-section">
      <div class="period-label">
        {#if latestStats}
          Latest: {formatPeriod(latestStats.period, $statistics.dimension)}
        {/if}
      </div>

      <div class="summary-cards">
        <!-- Min BPM Card -->
        <div class="stat-card">
          <div class="stat-label">Min BPM</div>
          <div class="stat-value min">{latestStats?.min_bpm ?? '--'}</div>
          {#if latestStats && previousStats}
            {@const trend = calculateTrend(latestStats.min_bpm, previousStats.min_bpm)}
            <div class="stat-trend {trend.direction}">
              {#if trend.direction === 'up'}
                <span class="trend-arrow">↑</span> +{trend.value.toFixed(1)}%
              {:else if trend.direction === 'down'}
                <span class="trend-arrow">↓</span> -{trend.value.toFixed(1)}%
              {:else}
                <span class="trend-stable">--</span>
              {/if}
            </div>
          {/if}
        </div>

        <!-- Max BPM Card -->
        <div class="stat-card">
          <div class="stat-label">Max BPM</div>
          <div class="stat-value max">{latestStats?.max_bpm ?? '--'}</div>
          {#if latestStats && previousStats}
            {@const trend = calculateTrend(latestStats.max_bpm, previousStats.max_bpm)}
            <div class="stat-trend {trend.direction}">
              {#if trend.direction === 'up'}
                <span class="trend-arrow">↑</span> +{trend.value.toFixed(1)}%
              {:else if trend.direction === 'down'}
                <span class="trend-arrow">↓</span> -{trend.value.toFixed(1)}%
              {:else}
                <span class="trend-stable">--</span>
              {/if}
            </div>
          {/if}
        </div>

        <!-- Avg BPM Card -->
        <div class="stat-card">
          <div class="stat-label">Avg BPM</div>
          <div class="stat-value avg">{latestStats?.avg_bpm.toFixed(1) ?? '--'}</div>
          {#if latestStats && previousStats}
            {@const trend = calculateTrend(latestStats.avg_bpm, previousStats.avg_bpm)}
            <div class="stat-trend {trend.direction}">
              {#if trend.direction === 'up'}
                <span class="trend-arrow">↑</span> +{trend.value.toFixed(1)}%
              {:else if trend.direction === 'down'}
                <span class="trend-arrow">↓</span> -{trend.value.toFixed(1)}%
              {:else}
                <span class="trend-stable">--</span>
              {/if}
            </div>
          {/if}
        </div>

        <!-- Record Count Card (D-11) -->
        <div class="stat-card">
          <div class="stat-label">Records</div>
          <div class="stat-value count">{latestStats?.record_count ?? '--'}</div>
        </div>

        <!-- HRV Estimation Card (D-09, D-11, D-12) -->
        <div class="stat-card hrv">
          <div class="stat-label">
            HRV
            {#if hrvData?.is_estimated}
              <span class="estimated-badge">(estimated)</span>
            {/if}
          </div>
          {#if hrvLoading}
            <div class="stat-value">...</div>
          {:else if hrvData}
            <div class="stat-value">{hrvData.hrv_value.toFixed(1)}</div>
            <div class="stat-unit">ms</div>
            <div class="hrv-confidence {hrvData.confidence}">
              {hrvData.confidence} confidence
            </div>
          {:else}
            <div class="stat-value">--</div>
            <div class="stat-unit">ms</div>
          {/if}
        </div>
      </div>
    </div>

    <!-- Trend Chart -->
    <div class="chart-section">
      <h4>Trend Analysis</h4>
      <StatisticsChart />
    </div>
  {/if}
</div>

<style>
  .statistics-tab {
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .dimension-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    flex-wrap: wrap;
    gap: 12px;
  }

  h3 {
    font-size: 18px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .dimension-pills {
    display: flex;
    gap: 6px;
  }

  .pill {
    padding: 6px 14px;
    border-radius: 12px;
    background: var(--bg-color);
    border: 1px solid var(--border-color);
    color: var(--text-secondary);
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .pill.active {
    background: var(--primary-color);
    color: white;
    border-color: var(--primary-color);
  }

  .pill:hover:not(.active) {
    border-color: var(--primary-color);
  }

  .loading-state,
  .error-state,
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 12px;
    padding: 48px;
    text-align: center;
    color: var(--text-muted);
    font-size: 14px;
  }

  .spinner {
    width: 24px;
    height: 24px;
    border: 2px solid var(--border-color);
    border-top-color: var(--primary-color);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .error-state {
    color: var(--danger-color);
  }

  .empty-state svg {
    opacity: 0.4;
  }

  .empty-state .hint {
    font-size: 12px;
    opacity: 0.7;
  }

  .summary-section {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .period-label {
    font-size: 13px;
    color: var(--text-secondary);
  }

  .summary-cards {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
    gap: 12px;
  }

  .stat-card {
    background: var(--bg-color);
    border-radius: 12px;
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .stat-label {
    font-size: 12px;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .stat-value {
    font-size: 28px;
    font-weight: 700;
    font-variant-numeric: tabular-nums;
  }

  .stat-value.min { color: var(--success-color); }
  .stat-value.max { color: var(--danger-color); }
  .stat-value.avg { color: var(--primary-color); }
  .stat-value.count { color: var(--text-secondary); font-size: 22px; }

  .estimated-badge {
    font-size: 10px;
    font-weight: 400;
    color: var(--text-muted);
    margin-left: 4px;
  }

  .hrv-confidence {
    font-size: 10px;
    margin-top: 4px;
  }

  .hrv-confidence.high { color: var(--success-color); }
  .hrv-confidence.medium { color: var(--warning-color); }
  .hrv-confidence.low { color: var(--text-muted); }

  .stat-card.hrv .stat-value {
    color: var(--primary-color);
  }

  .stat-trend {
    font-size: 11px;
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .stat-trend.up { color: var(--danger-color); }
  .stat-trend.down { color: var(--success-color); }
  .stat-trend.stable { color: var(--text-muted); }

  .trend-arrow {
    font-weight: 600;
  }

  .chart-section {
    margin-top: 8px;
  }

  h4 {
    font-size: 14px;
    font-weight: 600;
    color: var(--text-secondary);
    margin-bottom: 12px;
  }

  .chart-placeholder {
    text-align: center;
    color: var(--text-muted);
    font-size: 13px;
    padding: 40px;
    background: var(--bg-color);
    border-radius: 12px;
  }

  @media (max-width: 600px) {
    .dimension-header {
      flex-direction: column;
      align-items: flex-start;
    }

    .summary-cards {
      grid-template-columns: repeat(2, 1fr);
    }
  }
</style>
