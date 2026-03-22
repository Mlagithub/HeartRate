<script lang="ts">
  import { onMount } from 'svelte';
  import { exerciseTypes } from '$lib/stores/exerciseTypes';

  let newTypeName = '';
  let editingIndex = -1;
  let editValue = '';

  onMount(() => {
    exerciseTypes.loadTypes();
  });

  async function handleAdd() {
    const name = newTypeName.trim();
    if (name && !$exerciseTypes.includes(name)) {
      await exerciseTypes.addType(name);
      newTypeName = '';
    }
  }

  function startEdit(index: number) {
    editingIndex = index;
    editValue = $exerciseTypes[index];
  }

  async function saveEdit() {
    const newName = editValue.trim();
    if (newName && editingIndex >= 0) {
      await exerciseTypes.updateType($exerciseTypes[editingIndex], newName);
    }
    editingIndex = -1;
    editValue = '';
  }

  function cancelEdit() {
    editingIndex = -1;
    editValue = '';
  }

  async function handleDelete(index: number) {
    const name = $exerciseTypes[index];
    if (confirm(`Delete "${name}"?`)) {
      await exerciseTypes.deleteType(name);
    }
  }
</script>

<div class="types-editor">
  <div class="add-row">
    <input
      type="text"
      placeholder="New exercise type..."
      bind:value={newTypeName}
      on:keydown={(e) => e.key === 'Enter' && handleAdd()}
    />
    <button class="btn-add" on:click={handleAdd} disabled={!newTypeName.trim()}>Add</button>
  </div>

  <div class="types-list">
    {#each $exerciseTypes as type, index}
      <div class="type-row">
        {#if editingIndex === index}
          <input type="text" bind:value={editValue} on:keydown={(e) => e.key === 'Enter' && saveEdit()} />
          <button class="btn-save" on:click={saveEdit}>Save</button>
          <button class="btn-cancel" on:click={cancelEdit}>Cancel</button>
        {:else}
          <span class="type-name">{type}</span>
          <button class="btn-edit" on:click={() => startEdit(index)}>Edit</button>
          <button class="btn-delete" on:click={() => handleDelete(index)}>Delete</button>
        {/if}
      </div>
    {/each}
  </div>
</div>

<style>
  .types-editor { display: flex; flex-direction: column; gap: 12px; }
  .add-row { display: flex; gap: 8px; }
  .add-row input { flex: 1; padding: 8px 12px; border-radius: 8px; border: 1px solid var(--border-color); background: var(--bg-color); color: var(--text-primary); }
  .btn-add { padding: 8px 16px; background: var(--primary-color); color: white; border: none; border-radius: 8px; cursor: pointer; }
  .btn-add:disabled { opacity: 0.5; cursor: not-allowed; }
  .types-list { display: flex; flex-direction: column; gap: 8px; }
  .type-row { display: flex; align-items: center; gap: 8px; padding: 8px 12px; background: var(--bg-color); border-radius: 8px; }
  .type-name { flex: 1; }
  .type-row input { flex: 1; padding: 6px 10px; border-radius: 6px; border: 1px solid var(--border-color); background: var(--card-bg); color: var(--text-primary); }
  .btn-edit, .btn-delete, .btn-save, .btn-cancel { padding: 4px 12px; border-radius: 6px; font-size: 12px; cursor: pointer; border: 1px solid var(--border-color); background: transparent; color: var(--text-secondary); }
  .btn-edit:hover { border-color: var(--primary-color); color: var(--primary-color); }
  .btn-delete:hover { border-color: var(--danger-color); color: var(--danger-color); }
  .btn-save { background: var(--primary-color); color: white; border-color: var(--primary-color); }
</style>