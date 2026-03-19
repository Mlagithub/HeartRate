import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface HeartRateRecord {
  id: number | null;
  bpm: number;
  sensor_contact: boolean | null;
  timestamp: number;
  session_id: string;
}

export interface HistoryState {
  records: HeartRateRecord[];
  isLoading: boolean;
  error: string | null;
}

function createHistoryStore() {
  const initialState: HistoryState = {
    records: [],
    isLoading: false,
    error: null,
  };

  const { subscribe, update, set } = writable<HistoryState>(initialState);

  async function loadHistory(limit: number = 100, offset: number = 0) {
    update((state) => ({ ...state, isLoading: true, error: null }));
    try {
      const records = await invoke<HeartRateRecord[]>('get_heart_rate_history', {
        limit,
        offset,
      });
      update((state) => ({
        ...state,
        records,
        isLoading: false,
      }));
    } catch (error) {
      update((state) => ({
        ...state,
        isLoading: false,
        error: String(error),
      }));
    }
  }

  async function saveRecord(record: HeartRateRecord) {
    try {
      await invoke('save_heart_rate', {
        measurement: {
          bpm: record.bpm,
          sensor_contact: record.sensor_contact,
          timestamp: record.timestamp,
        },
        sessionId: record.session_id,
      });
    } catch (error) {
      console.error('Failed to save heart rate record:', error);
    }
  }

  return {
    subscribe,
    loadHistory,
    saveRecord,
    clear: () => {
      update((state) => ({
        ...state,
        records: [],
      }));
    },
    reset: () => set(initialState),
  };
}

export const history = createHistoryStore();