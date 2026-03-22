import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

function createExerciseTypesStore() {
  const { subscribe, set, update } = writable<string[]>([]);

  async function loadTypes() {
    try {
      const types = await invoke<string[]>('get_exercise_types');
      set(types);
    } catch (e) {
      console.error('Failed to load exercise types:', e);
    }
  }

  async function addType(name: string) {
    try {
      await invoke('add_exercise_type', { name });
      await loadTypes();
    } catch (e) {
      console.error('Failed to add exercise type:', e);
    }
  }

  async function updateType(oldName: string, newName: string) {
    try {
      await invoke('update_exercise_type', { oldName, newName });
      await loadTypes();
    } catch (e) {
      console.error('Failed to update exercise type:', e);
    }
  }

  async function deleteType(name: string) {
    try {
      await invoke('delete_exercise_type', { name });
      await loadTypes();
    } catch (e) {
      console.error('Failed to delete exercise type:', e);
    }
  }

  return { subscribe, loadTypes, addType, updateType, deleteType };
}

export const exerciseTypes = createExerciseTypesStore();