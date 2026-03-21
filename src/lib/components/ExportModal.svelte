<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { save } from '@tauri-apps/plugin-dialog';
  import { writeTextFile } from '@tauri-apps/plugin-fs';
  import { startOfDay, endOfDay, startOfWeek, endOfWeek } from 'date-fns';
  import type { HeartRateRecord } from '$lib/stores/history';

  export let onClose: () => void;
  const dispatch = createEventDispatcher();

  let exportFormat: 'csv' | 'json' = 'csv'; // D-16: CSV default
  let dateRange: 'today' | 'week' | 'all' | 'custom' = 'all';
  let customStartDate: string = '';
  let customEndDate: string = '';
  let isExporting = false;
  let error: string | null = null;

  async function handleExport() {
    isExporting = true;
    error = null;
    try {
      const records = await fetchRecords();
      const content = exportFormat === 'csv'
        ? recordsToCsv(records)
        : JSON.stringify(records, null, 2);

      const path = await save({
        filters: [{ name: exportFormat.toUpperCase(), extensions: [exportFormat] }],
        defaultPath: `heart-rate-export.${exportFormat}`
      });

      if (path) {
        await writeTextFile(path, content);
        dispatch('exported');
        onClose();
      }
    } catch (e) {
      error = String(e);
    } finally {
      isExporting = false;
    }
  }

  async function fetchRecords(): Promise<HeartRateRecord[]> {
    const now = Date.now();
    let start: number, end: number;

    switch (dateRange) {
      case 'today':
        start = startOfDay(new Date()).getTime();
        end = endOfDay(new Date()).getTime();
        break;
      case 'week':
        start = startOfWeek(new Date(), { weekStartsOn: 1 }).getTime();
        end = endOfWeek(new Date(), { weekStartsOn: 1 }).getTime();
        break;
      case 'custom':
        start = new Date(customStartDate).getTime();
        end = new Date(customEndDate).getTime() + 86400000; // Include end day
        break;
      default:
        start = 0;
        end = now;
    }

    return invoke<HeartRateRecord[]>('get_heart_rate_history_range', {
      start_time: start,
      end_time: end
    });
  }

  function recordsToCsv(records: HeartRateRecord[]): string {
    const headers = ['timestamp', 'bpm', 'session_id', 'sensor_contact'];
    const rows = records.map(r => [
      new Date(r.timestamp).toISOString(),
      r.bpm,
      r.session_id,
      r.sensor_contact ?? ''
    ].join(','));
    return [headers.join(','), ...rows].join('\n');
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') onClose();
  }
</script>

<svelte:window on:keydown={handleKeydown} />

<div class="modal-overlay" on:click={onClose} role="presentation">
  <div class="modal" on:click|stopPropagation role="dialog" aria-modal="true">
    <div class="modal-header">
      <h3>Export Heart Rate Data</h3>
      <button class="close-btn" on:click={onClose} aria-label="Close">&times;</button>
    </div>

    <div class="modal-body">
      <!-- Format Selection -->
      <div class="section">
        <label class="section-label">Format</label>
        <div class="format-options">
          <button
            class="format-btn"
            class:active={exportFormat === 'csv'}
            on:click={() => exportFormat = 'csv'}
          >CSV</button>
          <button
            class="format-btn"
            class:active={exportFormat === 'json'}
            on:click={() => exportFormat = 'json'}
          >JSON</button>
        </div>
      </div>

      <!-- Date Range -->
      <div class="section">
        <label class="section-label">Date Range</label>
        <div class="range-options">
          <button
            class="range-btn"
            class:active={dateRange === 'today'}
            on:click={() => dateRange = 'today'}
          >Today</button>
          <button
            class="range-btn"
            class:active={dateRange === 'week'}
            on:click={() => dateRange = 'week'}
          >This Week</button>
          <button
            class="range-btn"
            class:active={dateRange === 'all'}
            on:click={() => dateRange = 'all'}
          >All Time</button>
          <button
            class="range-btn"
            class:active={dateRange === 'custom'}
            on:click={() => dateRange = 'custom'}
          >Custom</button>
        </div>
        {#if dateRange === 'custom'}
          <div class="custom-dates">
            <input type="date" bind:value={customStartDate} />
            <span>to</span>
            <input type="date" bind:value={customEndDate} />
          </div>
        {/if}
      </div>

      {#if error}
        <div class="error-message">{error}</div>
      {/if}
    </div>

    <div class="modal-footer">
      <button class="btn-secondary" on:click={onClose}>Cancel</button>
      <button
        class="btn-primary"
        on:click={handleExport}
        disabled={isExporting || (dateRange === 'custom' && (!customStartDate || !customEndDate))}
      >
        {isExporting ? 'Exporting...' : 'Export'}
      </button>
    </div>
  </div>
</div>

<style>
  .modal-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.7);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal {
    background: var(--card-bg);
    border-radius: 16px;
    border: 1px solid var(--border-color);
    width: 90%;
    max-width: 420px;
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.4);
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 20px 24px;
    border-bottom: 1px solid var(--border-color);
  }

  .modal-header h3 {
    font-size: 18px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .close-btn {
    background: transparent;
    border: none;
    font-size: 24px;
    color: var(--text-muted);
    cursor: pointer;
    padding: 0;
    line-height: 1;
  }

  .modal-body {
    padding: 24px;
    display: flex;
    flex-direction: column;
    gap: 24px;
  }

  .section-label {
    display: block;
    font-size: 13px;
    font-weight: 500;
    color: var(--text-secondary);
    margin-bottom: 12px;
  }

  .format-options, .range-options {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
  }

  .format-btn, .range-btn {
    padding: 8px 16px;
    border-radius: 8px;
    background: var(--bg-color);
    border: 1px solid var(--border-color);
    color: var(--text-secondary);
    font-size: 13px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .format-btn.active, .range-btn.active {
    background: var(--primary-color);
    border-color: var(--primary-color);
    color: white;
  }

  .custom-dates {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-top: 12px;
  }

  .custom-dates input {
    flex: 1;
    padding: 10px 12px;
    border-radius: 8px;
    background: var(--bg-color);
    border: 1px solid var(--border-color);
    color: var(--text-primary);
    font-size: 14px;
  }

  .custom-dates span {
    color: var(--text-muted);
    font-size: 13px;
  }

  .error-message {
    padding: 12px;
    background: var(--danger-light);
    border-radius: 8px;
    color: var(--danger-color);
    font-size: 13px;
  }

  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
    padding: 16px 24px;
    border-top: 1px solid var(--border-color);
  }
</style>