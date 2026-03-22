<script lang="ts">
  import { onMount, onDestroy, tick } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import Chart from 'chart.js/auto';

  export let sessionId: string;
  export let onClose: () => void;

  interface HeartRateRecord {
    bpm: number;
    timestamp: number;
  }

  let canvas: HTMLCanvasElement;
  let chart: Chart | null = null;
  let isLoading = true;
  let error: string | null = null;
  let stats = { min: 0, max: 0, avg: 0, count: 0 };
  let records: HeartRateRecord[] = [];

  onMount(async () => {
    try {
      records = await invoke<HeartRateRecord[]>('get_session_records', { sessionId });
      if (records.length > 0) {
        calculateStats(records);
      }
      isLoading = false;
      // Wait for DOM to update
      await tick();
      // Small delay to ensure canvas is rendered
      setTimeout(() => {
        if (records.length > 0 && canvas) {
          initChart(records);
        }
      }, 50);
    } catch (e) {
      error = String(e);
      isLoading = false;
    }
  });

  onDestroy(() => {
    if (chart) {
      chart.destroy();
      chart = null;
    }
  });

  function calculateStats(records: HeartRateRecord[]) {
    const bpms = records.map(r => r.bpm);
    stats = {
      min: Math.min(...bpms),
      max: Math.max(...bpms),
      avg: Math.round(bpms.reduce((a, b) => a + b, 0) / bpms.length),
      count: records.length
    };
  }

  function initChart(records: HeartRateRecord[]) {
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const labels = records.map(r =>
      new Date(r.timestamp).toLocaleTimeString('en-US', {
        hour: '2-digit',
        minute: '2-digit',
        second: '2-digit',
      })
    );
    const data = records.map(r => r.bpm);

    chart = new Chart(ctx, {
      type: 'line',
      data: {
        labels,
        datasets: [{
          label: 'Heart Rate',
          data,
          borderColor: '#14b8a6',
          borderWidth: 2,
          backgroundColor: 'rgba(20, 184, 166, 0.1)',
          fill: true,
          tension: 0.4,
          pointRadius: 0,
        }],
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        plugins: { legend: { display: false } },
        scales: {
          x: { display: true, grid: { color: 'rgba(51, 65, 85, 0.5)' } },
          y: { display: true, grid: { color: 'rgba(51, 65, 85, 0.5)' } },
        },
      },
    });
  }
</script>

<div class="modal-overlay" on:click={onClose}>
  <div class="modal" on:click|stopPropagation>
    <div class="modal-header">
      <h3>Session Details</h3>
      <button class="close-btn" on:click={onClose}>×</button>
    </div>

    {#if isLoading}
      <div class="loading">Loading...</div>
    {:else if error}
      <div class="error">{error}</div>
    {:else}
      <div class="stats-row">
        <div class="stat"><span class="label">Min</span><span class="value">{stats.min}</span></div>
        <div class="stat"><span class="label">Avg</span><span class="value">{stats.avg}</span></div>
        <div class="stat"><span class="label">Max</span><span class="value">{stats.max}</span></div>
        <div class="stat"><span class="label">Count</span><span class="value">{stats.count}</span></div>
      </div>
      <div class="chart-wrapper">
        <canvas bind:this={canvas}></canvas>
      </div>
    {/if}
  </div>
</div>

<style>
  .modal-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.6);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 200;
  }
  .modal {
    background: var(--card-bg);
    border-radius: 12px;
    padding: 20px;
    width: 90%;
    max-width: 600px;
    max-height: 80vh;
    overflow-y: auto;
  }
  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
  }
  .close-btn {
    background: transparent;
    border: none;
    font-size: 24px;
    cursor: pointer;
    color: var(--text-secondary);
  }
  .stats-row {
    display: flex;
    gap: 16px;
    margin-bottom: 16px;
  }
  .stat {
    display: flex;
    flex-direction: column;
    align-items: center;
  }
  .label { font-size: 11px; color: var(--text-muted); }
  .value { font-size: 18px; font-weight: 600; }
  .chart-wrapper { height: 250px; }
  .loading, .error { text-align: center; padding: 40px; color: var(--text-secondary); }
  .error { color: var(--danger-color); }
</style>