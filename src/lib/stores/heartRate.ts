import { writable } from 'svelte/store';
import { onMount } from 'svelte';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';

export interface HeartRateMeasurement {
  bpm: number;
  sensor_contact: boolean | null;
  energy_expended: number | null;
  rr_intervals: number[];
  timestamp: number;
}

export interface HeartRateStats {
  min: number;
  max: number;
  avg: number;
  count: number;
}

export interface HeartRateState {
  current: HeartRateMeasurement | null;
  history: HeartRateMeasurement[];
  maxHistoryLength: number;
  stats: HeartRateStats;
}

function createHeartRateStore() {
  const initialStats: HeartRateStats = { min: 0, max: 0, avg: 0, count: 0 };
  const initialState: HeartRateState = {
    current: null,
    history: [],
    maxHistoryLength: 100,
    stats: initialStats,
  };

  const { subscribe, update, set } = writable<HeartRateState>(initialState);
  let unlisten: UnlistenFn | null = null;
  let sessionId: string = '';

  // Session-level stats (entire session, not just recent history)
  let sessionStats: HeartRateStats = { min: 0, max: 0, avg: 0, count: 0 };
  let totalBpm = 0;

  function generateSessionId(): string {
    return `session_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
  }

  function updateSessionStats(bpm: number) {
    if (sessionStats.count === 0) {
      sessionStats.min = bpm;
      sessionStats.max = bpm;
    } else {
      sessionStats.min = Math.min(sessionStats.min, bpm);
      sessionStats.max = Math.max(sessionStats.max, bpm);
    }
    totalBpm += bpm;
    sessionStats.count++;
    sessionStats.avg = Math.round(totalBpm / sessionStats.count);
    return { ...sessionStats };
  }

  async function saveMeasurement(measurement: HeartRateMeasurement) {
    if (!sessionId) {
      sessionId = generateSessionId();
    }
    try {
      await invoke('save_heart_rate', {
        measurement: {
          bpm: measurement.bpm,
          sensor_contact: measurement.sensor_contact,
          energy_expended: measurement.energy_expended,
          rr_intervals: measurement.rr_intervals,
          timestamp: measurement.timestamp,
        },
        sessionId: sessionId,
      });
      console.log('Saved heart rate:', measurement.bpm);
    } catch (error) {
      console.error('Failed to save heart rate measurement:', error);
    }
  }

  // Initialize listener - call this in onMount
  async function initListener() {
    if (typeof window !== 'undefined' && !unlisten) {
      unlisten = await listen<HeartRateMeasurement>('heart-rate-measurement', (event) => {
        const measurement = event.payload;
        const newStats = updateSessionStats(measurement.bpm);
        update((state) => {
          // History keeps only recent points for chart display
          const history = [...state.history, measurement].slice(-state.maxHistoryLength);
          return {
            ...state,
            current: measurement,
            history,
            stats: newStats,
          };
        });
        // Save to database
        saveMeasurement(measurement);
      });
    }
  }

  // Cleanup listener
  function cleanup() {
    if (unlisten) {
      unlisten();
      unlisten = null;
    }
  }

  return {
    subscribe,
    initListener,
    cleanup,
    addMeasurement: (measurement: HeartRateMeasurement) => {
      const newStats = updateSessionStats(measurement.bpm);
      update((state) => {
        const history = [...state.history, measurement].slice(-state.maxHistoryLength);
        return {
          ...state,
          current: measurement,
          history,
          stats: newStats,
        };
      });
    },
    clearHistory: () => {
      update((state) => ({
        ...state,
        history: [],
      }));
    },
    reset: () => {
      sessionStats = { min: 0, max: 0, avg: 0, count: 0 };
      totalBpm = 0;
      set(initialState);
    },
  };
}

export const heartRate = createHeartRateStore();