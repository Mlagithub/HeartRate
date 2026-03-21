import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

// Activity types per D-02
export const ACTIVITY_TYPES = ['Running', 'Cycling', 'Swimming', 'Gym', 'Other'] as const;
export type ActivityType = typeof ACTIVITY_TYPES[number];

export interface ExerciseTag {
  session_id: string;
  exercise_type: string;
  is_confirmed: boolean;
  confidence: number | null;
  tagged_at: number;
}

export interface SessionInfo {
  session_id: string;
  start_time: number;
  end_time: number;
  record_count: number;
  avg_bpm: number;
  exercise_tag: ExerciseTag | null;
}

export interface DetectionResult {
  session_id: string;
  is_exercise: boolean;
  confidence: number;
  reason: string;
}

export interface ExerciseState {
  sessions: SessionInfo[];
  detections: Map<string, DetectionResult>;
  isLoading: boolean;
  error: string | null;
}

function createExerciseStore() {
  const initialState: ExerciseState = {
    sessions: [],
    detections: new Map(),
    isLoading: false,
    error: null,
  };

  const { subscribe, update, set } = writable<ExerciseState>(initialState);

  async function loadSessions(limit: number = 50, offset: number = 0) {
    update((s) => ({ ...s, isLoading: true, error: null }));
    try {
      const sessions = await invoke<SessionInfo[]>('get_sessions_list', { limit, offset });
      update((s) => ({ ...s, sessions, isLoading: false }));
    } catch (error) {
      update((s) => ({ ...s, isLoading: false, error: String(error) }));
    }
  }

  async function tagExercise(
    sessionId: string,
    exerciseType: ActivityType,
    isConfirmed: boolean = true
  ) {
    try {
      await invoke('tag_exercise_session', {
        sessionId,
        exerciseType,
        isConfirmed,
      });
      // Refresh sessions after tagging
      await loadSessions();
    } catch (error) {
      update((s) => ({ ...s, error: String(error) }));
    }
  }

  async function detectExercise(sessionId: string): Promise<DetectionResult | null> {
    try {
      const result = await invoke<DetectionResult>('detect_exercise_session', { sessionId });
      update((s) => {
        const newDetections = new Map(s.detections);
        newDetections.set(sessionId, result);
        return { ...s, detections: newDetections };
      });
      return result;
    } catch (error) {
      update((s) => ({ ...s, error: String(error) }));
      return null;
    }
  }

  async function runDetectionForAll() {
    let currentSessions: SessionInfo[] = [];
    const unsubscribe = subscribe((s) => {
      currentSessions = s.sessions;
    });
    unsubscribe();

    // Detect exercise for all untagged sessions
    for (const session of currentSessions) {
      if (!session.exercise_tag) {
        await detectExercise(session.session_id);
      }
    }
  }

  return {
    subscribe,
    loadSessions,
    tagExercise,
    detectExercise,
    runDetectionForAll,
    reset: () => set(initialState),
  };
}

export const exercise = createExerciseStore();