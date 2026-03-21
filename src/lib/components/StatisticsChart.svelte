<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { statistics, type PeriodStats } from '$lib/stores/statistics';
  import Chart from 'chart.js/auto';
  import 'chartjs-adapter-date-fns';

  let canvas: HTMLCanvasElement;
  let chart: Chart | null = null;

  // Calculate 7-day moving average (D-10)
  // Per RESEARCH.md Pitfall 3: Start from 7th point for full window
  function calculateMovingAverage(data: { x: Date; y: number }[], window: number = 7) {
    return data.map((point, i, arr) => {
      if (i < window - 1) return null; // Not enough data points yet
      const sum = arr.slice(i - window + 1, i + 1).reduce((acc, p) => acc + p.y, 0);
      return { x: point.x, y: sum / window };
    }).filter((p): p is { x: Date; y: number } => p !== null);
  }

  // Parse period string to Date for time scale
  function parsePeriodDate(period: string, dimension: string): Date {
    if (dimension === 'daily') {
      return new Date(period); // YYYY-MM-DD
    } else if (dimension === 'weekly') {
      // YYYY-WW format: parse as start of week
      const [year, week] = period.split('-').map(Number);
      const date = new Date(year, 0, 1 + (week - 1) * 7);
      return date;
    } else if (dimension === 'monthly') {
      return new Date(period + '-01'); // YYYY-MM -> YYYY-MM-01
    } else {
      return new Date(period + '-01-01'); // YYYY -> YYYY-01-01
    }
  }

  function getTimeUnit(dimension: string): 'day' | 'week' | 'month' | 'year' {
    switch (dimension) {
      case 'daily': return 'day';
      case 'weekly': return 'week';
      case 'monthly': return 'month';
      case 'yearly': return 'year';
      default: return 'day';
    }
  }

  function getDisplayFormat(dimension: string): string {
    switch (dimension) {
      case 'daily': return 'MMM d';
      case 'weekly': return "'Week' W";
      case 'monthly': return 'MMM yyyy';
      case 'yearly': return 'yyyy';
      default: return 'MMM d';
    }
  }

  function initChart() {
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    chart = new Chart(ctx, {
      type: 'line',
      data: {
        datasets: [
          {
            label: 'Average BPM',
            data: [],
            borderColor: '#14b8a6',
            backgroundColor: 'rgba(20, 184, 166, 0.1)',
            fill: true,
            tension: 0.4,
            pointRadius: 4,
            pointHoverRadius: 6,
            pointBackgroundColor: '#14b8a6',
          },
          {
            label: '7-Day Moving Avg',
            data: [],
            borderColor: '#8b5cf6',
            borderWidth: 2,
            borderDash: [5, 5],
            pointRadius: 0,
            fill: false,
            tension: 0.4,
          },
        ],
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        interaction: {
          intersect: false,
          mode: 'index',
        },
        plugins: {
          legend: {
            display: true,
            position: 'top',
            labels: {
              color: '#94a3b8',
              usePointStyle: true,
              padding: 16,
            },
          },
          tooltip: {
            backgroundColor: 'rgba(15, 23, 42, 0.9)',
            titleColor: '#f1f5f9',
            bodyColor: '#f1f5f9',
            borderColor: '#334155',
            borderWidth: 1,
            padding: 12,
            cornerRadius: 8,
            displayColors: true,
          },
        },
        scales: {
          x: {
            type: 'time',
            time: {
              unit: 'day',
              displayFormats: { day: 'MMM d' },
            },
            grid: {
              color: 'rgba(51, 65, 85, 0.5)',
            },
            ticks: {
              color: '#94a3b8',
              maxTicksLimit: 10,
            },
          },
          y: {
            grid: {
              color: 'rgba(51, 65, 85, 0.5)',
            },
            ticks: {
              color: '#94a3b8',
            },
          },
        },
      },
    });
  }

  function updateChart() {
    if (!chart || !$statistics.stats || $statistics.stats.length === 0) return;

    const dimension = $statistics.dimension;
    const timeUnit = getTimeUnit(dimension);
    const displayFormat = getDisplayFormat(dimension);

    // Transform data for Chart.js time scale
    const avgData = $statistics.stats.map(s => ({
      x: parsePeriodDate(s.period, dimension).getTime(), // Use timestamp for time scale
      y: s.avg_bpm,
    }));

    // Calculate moving average (only for daily dimension with enough data)
    const movingAvgData = dimension === 'daily' && avgData.length >= 7
      ? calculateMovingAverage(avgData.map(d => ({ x: new Date(d.x), y: d.y }))).map(d => ({ x: d.x.getTime(), y: d.y }))
      : [];

    // Update chart data
    chart.data.datasets[0].data = avgData as unknown as number[];
    chart.data.datasets[1].data = movingAvgData as unknown as number[];

    // Update time scale based on dimension - use type assertion for time scale
    const xScale = chart.options.scales!.x as { time: { unit: string; displayFormats: Record<string, string> } };
    xScale.time.unit = timeUnit;
    xScale.time.displayFormats = {
      [timeUnit]: displayFormat
    };

    chart.update();
  }

  let unsubscribe: (() => void) | null = null;

  onMount(() => {
    initChart();
    unsubscribe = statistics.subscribe(updateChart);
  });

  onDestroy(() => {
    if (unsubscribe) unsubscribe();
    if (chart) chart.destroy();
  });
</script>

<div class="chart-container">
  {#if $statistics.stats.length === 0 && !$statistics.isLoading}
    <div class="no-data">
      <p>No data to display</p>
    </div>
  {:else}
    <canvas bind:this={canvas}></canvas>
  {/if}
</div>

<style>
  .chart-container {
    height: 280px;
    position: relative;
    background: var(--bg-color);
    border-radius: 12px;
    padding: 16px;
  }

  .no-data {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: var(--text-muted);
    font-size: 14px;
  }
</style>
