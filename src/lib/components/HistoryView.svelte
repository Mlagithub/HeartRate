<script lang="ts">
  import { onMount } from 'svelte';
  import { history } from '$lib/stores/history';
  import { format } from 'date-fns';
  import ExportModal from './ExportModal.svelte';
  import StatisticsTab from './StatisticsTab.svelte';

  // Tab state (D-01, D-04)
  let activeTab: 'history' | 'statistics' = 'history';

  // History tab state
  onMount(() => {
    history.loadHistory(50);
  });

  let showExportModal = false;

  function formatTimestamp(ts: number): string {
    return format(new Date(ts), 'MMM d, yyyy HH:mm:ss');
  }

  function getBpmClass(bpm: number): string {
    if (bpm < 60) return 'low';
    if (bpm < 100) return 'normal';
    if (bpm < 140) return 'elevated';
    return 'high';
  }
</script>

<div class="analytics-view glass-card">
  <!-- Tab Navigation (D-01, D-04) -->
  <div class="tab-navigation">
    <button
      class="tab-button"
      class:active={activeTab === 'history'}
      on:click={() => activeTab = 'history'}
    >
      <svg viewBox="0 0 24 24" fill="currentColor" width="16" height="16">
        <path d="M13 3c-4.97 0-9 4.03-9 9H1l3.89 3.89.07.14L9 12H6c0-3.87 3.13-7 7-7s7 3.13 7 7-3.13 7-7 7c-1.93 0-3.68-.79-4.94-2.06l-1.42 1.42A8.954 8.954 0 0 0 13 21a9 9 0 0 0 0-18zm-1 5v5l4.28 2.54.72-1.21-3.5-2.08V8H12z"/>
      </svg>
      History
    </button>
    <button
      class="tab-button"
      class:active={activeTab === 'statistics'}
      on:click={() => activeTab = 'statistics'}
    >
      <svg viewBox="0 0 24 24" fill="currentColor" width="16" height="16">
        <path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V5h14v14zM7 10h2v7H7zm4-3h2v10h-2zm4 6h2v4h-2z"/>
      </svg>
      Statistics
    </button>
  </div>

  <!-- Tab Content -->
  {#if activeTab === 'history'}
    <!-- History Tab Content (D-03: preserve existing functionality) -->
    <div class="history-tab">
      <div class="history-header">
        <div class="header-left">
          <svg viewBox="0 0 24 24" fill="currentColor" width="24" height="24">
            <path d="M13 3c-4.97 0-9 4.03-9 9H1l3.89 3.89.07.14L9 12H6c0-3.87 3.13-7 7-7s7 3.13 7 7-3.13 7-7 7c-1.93 0-3.68-.79-4.94-2.06l-1.42 1.42A8.954 8.954 0 0 0 13 21a9 9 0 0 0 0-18zm-1 5v5l4.28 2.54.72-1.21-3.5-2.08V8H12z"/>
          </svg>
          <h3>Recent Records</h3>
        </div>
        <div class="header-buttons">
          <button
            class="btn-refresh"
            on:click={() => history.loadHistory(50)}
          >
            <svg viewBox="0 0 24 24" fill="currentColor" width="14" height="14">
              <path d="M17.65 6.35A7.958 7.958 0 0 0 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08A5.99 5.99 0 0 1 12 18c-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/>
            </svg>
            Refresh
          </button>
          <button
            class="btn-export"
            on:click={() => showExportModal = true}
          >
            <svg viewBox="0 0 24 24" fill="currentColor" width="14" height="14">
              <path d="M19 9h-4V3H9v6H5l7 7 7-7zM5 18v2h14v-2H5z"/>
            </svg>
            Export
          </button>
        </div>
      </div>

      {#if $history.isLoading}
        <div class="loading-state">
          <div class="spinner"></div>
          <span>Loading history...</span>
        </div>
      {:else if $history.error}
        <div class="error-state">
          <svg viewBox="0 0 24 24" fill="currentColor" width="20" height="20">
            <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-2h2v2zm0-4h-2V7h2v6z"/>
          </svg>
          <span>Error: {$history.error}</span>
        </div>
      {:else if $history.records.length === 0}
        <div class="empty-state">
          <svg viewBox="0 0 24 24" fill="currentColor" width="48" height="48">
            <path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V5h14v14zM7 10h2v7H7zm4-3h2v10h-2zm4 6h2v4h-2z"/>
          </svg>
          <span>No history records found</span>
          <span class="hint">Connect a device to start recording heart rate data</span>
        </div>
      {:else}
        <div class="records-list">
          {#each $history.records as record (record.id)}
            <div class="record">
              <div class="record-time">
                <svg viewBox="0 0 24 24" fill="currentColor" width="14" height="14">
                  <path d="M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8zm.5-13H11v6l5.25 3.15.75-1.23-4.5-2.67z"/>
                </svg>
                {formatTimestamp(record.timestamp)}
              </div>
              <div class="record-bpm {getBpmClass(record.bpm)}">
                <span class="bpm-value">{record.bpm}</span>
                <span class="bpm-unit">BPM</span>
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  {:else}
    <!-- Statistics Tab Content -->
    <StatisticsTab />
  {/if}
</div>

{#if showExportModal}
  <ExportModal onClose={() => showExportModal = false} />
{/if}

<style>
  .analytics-view {
    padding: 24px;
  }

  .tab-navigation {
    display: flex;
    gap: 8px;
    margin-bottom: 20px;
    padding-bottom: 16px;
    border-bottom: 1px solid var(--border-color);
  }

  .tab-button {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 10px 18px;
    background: transparent;
    color: var(--text-secondary);
    font-size: 14px;
    font-weight: 500;
    border: 1px solid transparent;
    border-radius: 10px;
    transition: all 0.2s ease;
  }

  .tab-button:hover {
    background: var(--bg-color);
    color: var(--text-primary);
  }

  .tab-button.active {
    background: var(--primary-color);
    color: white;
    border-color: var(--primary-color);
  }

  .tab-button svg {
    opacity: 0.8;
  }

  /* History tab styles (preserve existing) */
  .history-tab {
    /* No additional styles needed, content fills the space */
  }

  .history-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;
  }

  .header-left {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .header-left svg {
    color: var(--primary-color);
  }

  h3 {
    font-size: 18px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .btn-refresh {
    display: flex;
    align-items: center;
    gap: 6px;
    background: transparent;
    color: var(--text-secondary);
    font-size: 13px;
    padding: 8px 14px;
    border: 1px solid var(--border-color);
    border-radius: 8px;
    transition: all 0.2s ease;
  }

  .btn-refresh:hover {
    border-color: var(--primary-color);
    color: var(--primary-color);
  }

  .header-buttons {
    display: flex;
    gap: 8px;
  }

  .btn-export {
    display: flex;
    align-items: center;
    gap: 6px;
    background: transparent;
    color: var(--text-secondary);
    font-size: 13px;
    padding: 8px 14px;
    border: 1px solid var(--border-color);
    border-radius: 8px;
    transition: all 0.2s ease;
  }

  .btn-export:hover {
    border-color: var(--primary-color);
    color: var(--primary-color);
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

  .records-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
    max-height: 400px;
    overflow-y: auto;
    padding-right: 4px;
  }

  .record {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 14px 16px;
    background: var(--bg-color);
    border-radius: 10px;
    transition: all 0.2s ease;
  }

  .record:hover {
    background: var(--card-bg-hover);
  }

  .record-time {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 13px;
    color: var(--text-secondary);
  }

  .record-time svg {
    opacity: 0.6;
  }

  .record-bpm {
    display: flex;
    align-items: baseline;
    gap: 4px;
    padding: 6px 12px;
    border-radius: 8px;
  }

  .record-bpm.low {
    background: var(--success-light);
  }

  .record-bpm.low .bpm-value {
    color: var(--success-color);
  }

  .record-bpm.normal {
    background: var(--primary-light);
  }

  .record-bpm.normal .bpm-value {
    color: var(--primary-color);
  }

  .record-bpm.elevated {
    background: var(--warning-light);
  }

  .record-bpm.elevated .bpm-value {
    color: var(--warning-color);
  }

  .record-bpm.high {
    background: var(--danger-light);
  }

  .record-bpm.high .bpm-value {
    color: var(--danger-color);
  }

  .bpm-value {
    font-size: 16px;
    font-weight: 700;
    font-variant-numeric: tabular-nums;
  }

  .bpm-unit {
    font-size: 11px;
    color: var(--text-muted);
  }
</style>