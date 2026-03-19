<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { heartRate } from '$lib/stores/heartRate';
  import Chart from 'chart.js/auto';

  let canvas: HTMLCanvasElement;
  let chart: Chart | null = null;
  let showZones = true;
  let lastUpdate = 0;
  const UPDATE_THROTTLE_MS = 250; // Throttle chart updates

  const MAX_POINTS = 120; // 2 minutes of data

  // Zone backgrounds for chart
  const zoneColors = [
    { min: 0, max: 60, color: 'rgba(148, 163, 184, 0.1)' },
    { min: 60, max: 100, color: 'rgba(34, 197, 94, 0.1)' },
    { min: 100, max: 140, color: 'rgba(245, 158, 11, 0.1)' },
    { min: 140, max: 170, color: 'rgba(239, 68, 68, 0.1)' },
    { min: 170, max: 220, color: 'rgba(220, 38, 38, 0.1)' },
  ];

  function getBpmColor(bpm: number): string {
    if (bpm < 60) return '#94a3b8';
    if (bpm < 100) return '#22c55e';
    if (bpm < 140) return '#f59e0b';
    if (bpm < 170) return '#ef4444';
    return '#dc2626';
  }

  function initChart() {
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    chart = new Chart(ctx, {
      type: 'line',
      data: {
        labels: [],
        datasets: [
          {
            label: 'Heart Rate',
            data: [],
            borderColor: '#3b82f6',
            borderWidth: 2,
            backgroundColor: 'rgba(59, 130, 246, 0.1)',
            fill: true,
            tension: 0.4,
            pointRadius: 0,
            pointHoverRadius: 6,
            pointHoverBackgroundColor: '#3b82f6',
            pointHoverBorderColor: '#ffffff',
            pointHoverBorderWidth: 2,
          },
        ],
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        animation: {
          duration: 200,
        },
        interaction: {
          intersect: false,
          mode: 'index',
        },
        plugins: {
          legend: {
            display: false,
          },
          tooltip: {
            backgroundColor: 'rgba(15, 23, 42, 0.9)',
            titleColor: '#f1f5f9',
            bodyColor: '#f1f5f9',
            borderColor: '#334155',
            borderWidth: 1,
            padding: 12,
            cornerRadius: 8,
            displayColors: false,
            callbacks: {
              label: (context) => `${context.parsed.y} BPM`,
            },
          },
        },
        scales: {
          x: {
            display: true,
            grid: {
              color: 'rgba(51, 65, 85, 0.5)',
            },
            ticks: {
              color: '#94a3b8',
              maxTicksLimit: 6,
              font: { size: 11 },
            },
          },
          y: {
            display: true,
            grid: {
              color: 'rgba(51, 65, 85, 0.5)',
            },
            ticks: {
              color: '#94a3b8',
              font: { size: 11 },
            },
          },
        },
      },
      plugins: [
        {
          id: 'zoneBackgrounds',
          beforeDraw: (chart) => {
            if (!showZones) return;
            const ctx = chart.ctx;
            const yAxis = chart.scales.y;
            const chartArea = chart.chartArea;

            // Get current Y-axis range
            const yMin = yAxis.min;
            const yMax = yAxis.max;

            ctx.save();
            zoneColors.forEach(zone => {
              // Only draw zones that overlap with visible range
              const zoneTop = Math.min(zone.max, yMax);
              const zoneBottom = Math.max(zone.min, yMin);

              if (zoneTop > zoneBottom) {
                const yTop = yAxis.getPixelForValue(zoneTop);
                const yBottom = yAxis.getPixelForValue(zoneBottom);
                ctx.fillStyle = zone.color;
                ctx.fillRect(chartArea.left, yTop, chartArea.right - chartArea.left, yBottom - yTop);
              }
            });
            ctx.restore();
          },
        },
      ],
    });
  }

  function calculateYAxisRange(data: number[]): { min: number; max: number } {
    if (data.length === 0) {
      return { min: 40, max: 200 };
    }

    const dataMin = Math.min(...data);
    const dataMax = Math.max(...data);

    // Add 10% padding on each side, with minimum range of 20 BPM
    const range = dataMax - dataMin;
    const padding = Math.max(range * 0.15, 10);

    // Round to nearest 10 for clean axis labels
    const min = Math.max(30, Math.floor((dataMin - padding) / 10) * 10);
    const max = Math.min(220, Math.ceil((dataMax + padding) / 10) * 10);

    return { min, max };
  }

  function updateChart() {
    if (!chart) return;

    // Throttle updates to avoid excessive redraws
    const now = Date.now();
    if (now - lastUpdate < UPDATE_THROTTLE_MS) return;
    lastUpdate = now;

    const history = $heartRate.history.slice(-MAX_POINTS);
    const labels = history.map((h) =>
      new Date(h.timestamp).toLocaleTimeString('en-US', {
        hour: '2-digit',
        minute: '2-digit',
        second: '2-digit',
      })
    );
    const data = history.map((h) => h.bpm);
    const colors = history.map((h) => getBpmColor(h.bpm));

    // Calculate dynamic Y-axis range
    const { min: yMin, max: yMax } = calculateYAxisRange(data);

    chart.data.labels = labels;
    chart.data.datasets[0].data = data;
    // Update point colors if the dataset supports it
    if ('backgroundColor' in chart.data.datasets[0]) {
      chart.data.datasets[0].backgroundColor = colors;
    }

    // Update Y-axis scale dynamically
    chart.options.scales!.y!.min = yMin;
    chart.options.scales!.y!.max = yMax;

    chart.update('none');
  }

  let unsubscribe: (() => void) | null = null;

  onMount(() => {
    initChart();
    unsubscribe = heartRate.subscribe(updateChart);
  });

  onDestroy(() => {
    if (unsubscribe) unsubscribe();
    if (chart) {
      chart.destroy();
      chart = null;
    }
  });
</script>

<div class="chart-container glass-card">
  <div class="chart-header">
    <h3>Heart Rate Trend</h3>
    <label class="toggle-label">
      <input type="checkbox" bind:checked={showZones} />
      <span class="toggle-text">Show Zones</span>
    </label>
  </div>

  <div class="chart-wrapper">
    <canvas bind:this={canvas}></canvas>
    {#if $heartRate.history.length === 0}
      <div class="no-data">
        <div class="no-data-icon">📊</div>
        <div>Connect a device to start recording</div>
      </div>
    {/if}
  </div>

  <!-- Zone Legend -->
  <div class="zone-legend">
    <div class="legend-item"><span class="dot rest"></span> Rest (&lt;60)</div>
    <div class="legend-item"><span class="dot fat-burn"></span> Fat Burn (60-100)</div>
    <div class="legend-item"><span class="dot cardio"></span> Cardio (100-140)</div>
    <div class="legend-item"><span class="dot peak"></span> Peak (140-170)</div>
    <div class="legend-item"><span class="dot extreme"></span> Extreme (&gt;170)</div>
  </div>
</div>

<style>
  .chart-container {
    padding: 20px;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .chart-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  h3 {
    font-size: 16px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .toggle-label {
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
    font-size: 13px;
    color: var(--text-secondary);
  }

  .toggle-label input {
    accent-color: var(--primary-color);
  }

  .chart-wrapper {
    height: 250px;
    position: relative;
  }

  .no-data {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    text-align: center;
    color: var(--text-muted);
    font-size: 14px;
    pointer-events: none;
  }

  .no-data-icon {
    font-size: 32px;
    margin-bottom: 8px;
    opacity: 0.5;
  }

  .zone-legend {
    display: flex;
    flex-wrap: wrap;
    gap: 12px;
    padding-top: 12px;
    border-top: 1px solid var(--border-color);
    font-size: 11px;
    color: var(--text-muted);
  }

  .legend-item {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
  }

  .dot.rest { background: #94a3b8; }
  .dot.fat-burn { background: #22c55e; }
  .dot.cardio { background: #f59e0b; }
  .dot.peak { background: #ef4444; }
  .dot.extreme { background: #dc2626; }

  @media (max-width: 600px) {
    .chart-wrapper {
      height: 200px;
    }

    .zone-legend {
      justify-content: center;
    }
  }
</style>