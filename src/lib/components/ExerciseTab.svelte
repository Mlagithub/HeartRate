<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';

  // Types matching Rust backend (from 03-04 PLAN)
  interface HeartRateZones {
    zone1_pct: number;
    zone2_pct: number;
    zone3_pct: number;
    zone4_pct: number;
    zone5_pct: number;
  }

  interface ExerciseStats {
    avg_exercise_hr: number;
    avg_resting_hr: number;
    exercise_sessions: number;
    total_exercise_minutes: number;
    hr_zones: HeartRateZones;
  }

  interface ExerciseTypeStats {
    exercise_type: string;
    session_count: number;
    avg_hr: number;
    max_hr: number;
    min_hr: number;
    total_minutes: number;
  }

  let stats: ExerciseStats | null = null;
  let typeStats: ExerciseTypeStats[] = [];
  let isLoading = true;
  let error: string | null = null;

  onMount(async () => {
    // Load with a small delay to avoid blocking
    setTimeout(() => {
      loadExerciseStats();
    }, 200);
  });

  async function loadExerciseStats() {
    isLoading = true;
    error = null;
    try {
      stats = await invoke<ExerciseStats>('get_exercise_statistics');
      typeStats = await invoke<ExerciseTypeStats[]>('get_exercise_type_statistics');
    } catch (e) {
      error = String(e);
    }
    isLoading = false;
  }
</script>

<div class="exercise-tab">
  <!-- Loading State -->
  {#if isLoading}
    <div class="loading-state">
      <div class="spinner"></div>
      <span>Loading exercise statistics...</span>
    </div>

  <!-- Error State -->
  {:else if error}
    <div class="error-state">
      <svg viewBox="0 0 24 24" fill="currentColor" width="20" height="20">
        <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-2h2v2zm0-4h-2V7h2v6z"/>
      </svg>
      <span>Error: {error}</span>
    </div>

  <!-- Empty State -->
  {:else if !stats || stats.exercise_sessions === 0}
    <div class="empty-state">
      <svg viewBox="0 0 24 24" fill="currentColor" width="48" height="48">
        <path d="M13.49 5.48c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm-3.6 13.9l1-4.4 2.1 2v6h2v-7.5l-2.1-2 .6-3c1.3 1.5 3.3 2.5 5.5 2.5v-2c-1.9 0-3.5-1-4.3-2.4l-1-1.6c-.4-.6-1-1-1.7-1-.3 0-.5.1-.8.1l-5.2 2.2v4.7h2v-3.4l1.8-.7-1.6 8.1-4.9-1-.4 2 7 1.4z"/>
      </svg>
      <span>No exercise sessions recorded</span>
      <span class="hint">Tag sessions as exercise in the History tab to see analytics</span>
    </div>

  <!-- Exercise Content -->
  {:else}
    <!-- Exercise vs Resting Comparison (D-14) -->
    <div class="comparison-section">
      <h4>Exercise vs Resting Heart Rate</h4>
      <div class="comparison-cards">
        <div class="stat-card exercise">
          <div class="stat-label">Avg Exercise HR</div>
          <div class="stat-value">{stats.avg_exercise_hr.toFixed(0)}</div>
          <div class="stat-unit">BPM</div>
        </div>
        <div class="stat-card resting">
          <div class="stat-label">Avg Resting HR</div>
          <div class="stat-value">{stats.avg_resting_hr.toFixed(0)}</div>
          <div class="stat-unit">BPM</div>
        </div>
        <div class="stat-card difference">
          <div class="stat-label">Difference</div>
          <div class="stat-value">{(stats.avg_exercise_hr - stats.avg_resting_hr).toFixed(0)}</div>
          <div class="stat-unit">BPM higher</div>
        </div>
      </div>
    </div>

    <!-- HR Zones Visualization (D-15) -->
    <div class="zones-section">
      <h4>Heart Rate Zones</h4>
      <div class="zones-bar">
        {#if stats.hr_zones}
          {#if stats.hr_zones.zone1_pct > 0}
            <div class="zone zone1" style="width: {stats.hr_zones.zone1_pct}%">Zone 1</div>
          {/if}
          {#if stats.hr_zones.zone2_pct > 0}
            <div class="zone zone2" style="width: {stats.hr_zones.zone2_pct}%">Zone 2</div>
          {/if}
          {#if stats.hr_zones.zone3_pct > 0}
            <div class="zone zone3" style="width: {stats.hr_zones.zone3_pct}%">Zone 3</div>
          {/if}
          {#if stats.hr_zones.zone4_pct > 0}
            <div class="zone zone4" style="width: {stats.hr_zones.zone4_pct}%">Zone 4</div>
          {/if}
          {#if stats.hr_zones.zone5_pct > 0}
            <div class="zone zone5" style="width: {stats.hr_zones.zone5_pct}%">Zone 5</div>
          {/if}
        {/if}
      </div>
      <div class="zones-legend">
        <span class="zone1">50-60%</span>
        <span class="zone2">60-70%</span>
        <span class="zone3">70-80%</span>
        <span class="zone4">80-90%</span>
        <span class="zone5">90-100%</span>
      </div>
    </div>

    <!-- Exercise Type Comparison (D-16, STAT-08) -->
    <div class="types-section">
      <h4>By Exercise Type</h4>
      {#if typeStats.length === 0}
        <div class="empty-types">No exercise sessions tagged yet</div>
      {:else}
        <div class="types-table">
          <div class="table-header">
            <span>Type</span>
            <span>Sessions</span>
            <span>Avg HR</span>
            <span>Max HR</span>
            <span>Duration</span>
          </div>
          {#each typeStats as type}
            <div class="table-row">
              <span class="type-name">{type.exercise_type}</span>
              <span>{type.session_count}</span>
              <span>{type.avg_hr.toFixed(0)}</span>
              <span>{type.max_hr}</span>
              <span>{Math.round(type.total_minutes)} min</span>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .exercise-tab {
    display: flex;
    flex-direction: column;
    gap: 24px;
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

  h4 {
    font-size: 14px;
    font-weight: 600;
    color: var(--text-secondary);
    margin-bottom: 12px;
  }

  /* Comparison Cards */
  .comparison-cards {
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

  .stat-unit {
    font-size: 11px;
    color: var(--text-muted);
  }

  .stat-card.exercise .stat-value { color: var(--danger-color); }
  .stat-card.resting .stat-value { color: var(--success-color); }
  .stat-card.difference .stat-value { color: var(--primary-color); }

  /* HR Zones */
  .zones-bar {
    display: flex;
    height: 32px;
    border-radius: 8px;
    overflow: hidden;
    background: var(--bg-color);
  }

  .zone {
    display: flex;
    align-items: center;
    justify-content: center;
    color: white;
    font-size: 11px;
    font-weight: 600;
    min-width: 20px;
    transition: width 0.3s ease;
  }

  .zone1 { background: #10b981; }
  .zone2 { background: #14b8a6; }
  .zone3 { background: #f59e0b; }
  .zone4 { background: #ef4444; }
  .zone5 { background: #7c3aed; }

  .zones-legend {
    display: flex;
    justify-content: space-between;
    margin-top: 8px;
    font-size: 10px;
    color: var(--text-muted);
  }

  .zones-legend span {
    padding: 2px 6px;
    border-radius: 4px;
    color: white;
  }

  .zones-legend .zone1 { background: #10b981; }
  .zones-legend .zone2 { background: #14b8a6; }
  .zones-legend .zone3 { background: #f59e0b; }
  .zones-legend .zone4 { background: #ef4444; }
  .zones-legend .zone5 { background: #7c3aed; }

  /* Types Table */
  .types-table {
    background: var(--bg-color);
    border-radius: 12px;
    overflow: hidden;
  }

  .table-header,
  .table-row {
    display: grid;
    grid-template-columns: 2fr 1fr 1fr 1fr 1fr;
    padding: 12px 16px;
    align-items: center;
  }

  .table-header {
    background: var(--card-bg-hover);
    font-size: 11px;
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .table-row {
    border-top: 1px solid var(--border-color);
    font-size: 13px;
    color: var(--text-primary);
  }

  .table-row:hover {
    background: var(--card-bg-hover);
  }

  .type-name {
    font-weight: 500;
    color: var(--primary-color);
  }

  .empty-types {
    text-align: center;
    padding: 24px;
    color: var(--text-muted);
    font-size: 13px;
    background: var(--bg-color);
    border-radius: 12px;
  }
</style>