import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface PeriodStats {
  period: string;
  min_bpm: number;
  max_bpm: number;
  avg_bpm: number;
  record_count: number;
}

export interface StatisticsState {
  stats: PeriodStats[];
  isLoading: boolean;
  error: string | null;
  dimension: 'daily' | 'weekly' | 'monthly' | 'yearly';
}

function createStatisticsStore() {
  const initialState: StatisticsState = {
    stats: [],
    isLoading: false,
    error: null,
    dimension: 'weekly', // Per D-06: Weekly view as default
  };

  const { subscribe, update, set } = writable<StatisticsState>(initialState);

  async function loadStatistics(
    dimension: 'daily' | 'weekly' | 'monthly' | 'yearly',
    startTime?: number,
    endTime?: number
  ) {
    update((state) => ({ ...state, isLoading: true, error: null, dimension }));
    try {
      const stats = await invoke<PeriodStats[]>('get_heart_rate_statistics', {
        dimension,
        startTime,
        endTime,
      });
      update((state) => ({ ...state, stats, isLoading: false }));
    } catch (error) {
      update((state) => ({
        ...state,
        isLoading: false,
        error: String(error),
      }));
    }
  }

  return {
    subscribe,
    loadStatistics,
    reset: () => set(initialState),
  };
}

export const statistics = createStatisticsStore();
