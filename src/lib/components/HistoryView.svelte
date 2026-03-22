<script lang="ts">
  import { onMount } from 'svelte';
  import { history } from '$lib/stores/history';
  import { exercise, type DetectionResult } from '$lib/stores/exercise';
  import { exerciseTypes } from '$lib/stores/exerciseTypes';
  import { format, formatDuration, intervalToDuration } from 'date-fns';
  import ExportModal from './ExportModal.svelte';
  import StatisticsTab from './StatisticsTab.svelte';
  import ExerciseTab from './ExerciseTab.svelte';
  import SessionChart from './SessionChart.svelte';

  // Tab state (D-01, D-04, D-13)
  let activeTab: 'history' | 'statistics' | 'exercise' = 'history';

  // Track if data has been loaded
  let dataLoaded = false;

  // Session chart modal
  let showSessionChart = false;
  let selectedSessionId = '';

  // History tab state - only load once
  onMount(() => {
    if (!dataLoaded) {
      console.log('[HistoryView] Loading data for the first time...');
      dataLoaded = true;
      // Load with a small delay to avoid blocking
      setTimeout(() => {
        history.loadHistory(50);
        exercise.loadSessions(50);
        exerciseTypes.loadTypes();
      }, 100);
    }
  });

  let showExportModal = false;

  // Exercise tagging state
  let expandedSession: string | null = null;
  let selectedActivity: string = 'Running';
  let editingTag: string | null = null;
  let editActivity: string = '';

  function formatTimestamp(ts: number): string {
    return format(new Date(ts), 'MMM d, yyyy HH:mm:ss');
  }

  function formatSessionTime(ts: number): string {
    return format(new Date(ts), 'MMM d, HH:mm');
  }

  function formatDurationMs(start: number, end: number): string {
    const duration = intervalToDuration({ start: new Date(start), end: new Date(end) });
    if (duration.hours && duration.hours > 0) {
      return `${duration.hours}h ${duration.minutes ?? 0}m`;
    }
    return `${duration.minutes ?? 0}m`;
  }

  function getBpmClass(bpm: number): string {
    if (bpm < 60) return 'low';
    if (bpm < 100) return 'normal';
    if (bpm < 140) return 'elevated';
    return 'high';
  }

  function toggleSession(sessionId: string) {
    expandedSession = expandedSession === sessionId ? null : sessionId;
  }

  async function handleTagExercise(sessionId: string) {
    await exercise.tagExercise(sessionId, selectedActivity, true);
    expandedSession = null;
  }

  async function confirmDetection(sessionId: string, detection: DetectionResult) {
    // Map detection reason to activity type heuristically
    const detectedType: string = detection.reason.toLowerCase().includes('running')
      ? 'Running'
      : detection.reason.toLowerCase().includes('cycling')
        ? 'Cycling'
        : detection.reason.toLowerCase().includes('swimming')
          ? 'Swimming'
          : detection.reason.toLowerCase().includes('gym')
            ? 'Gym'
            : 'Other';
    await exercise.tagExercise(sessionId, detectedType, true);
  }

  async function dismissDetection(sessionId: string) {
    await exercise.tagExercise(sessionId, 'Other', false);
  }

  async function handleDeleteSession(sessionId: string) {
    if (confirm('Are you sure you want to delete this session? This action cannot be undone.')) {
      await exercise.deleteSession(sessionId);
      expandedSession = null;
    }
  }

  function startEditTag(sessionId: string, currentType: string) {
    editingTag = sessionId;
    editActivity = currentType;
  }

  function cancelEditTag() {
    editingTag = null;
    editActivity = '';
  }

  async function saveEditTag(sessionId: string) {
    await exercise.tagExercise(sessionId, editActivity, true);
    editingTag = null;
    editActivity = '';
  }

  async function removeTag(sessionId: string) {
    if (confirm('Remove exercise tag from this session?')) {
      await exercise.removeTag(sessionId);
    }
  }
</script>

<div class="analytics-view glass-card">
  <!-- Tab Navigation (D-01, D-04) -->
  <div class="tab-navigation">
    <button
      class="tab-button"
      class:active={activeTab === 'history'}
      on:click={() => activeTab = 'history'}
    >
      <svg viewBox="0 0 24 24" fill="currentColor" width="16" height="16">
        <path d="M13 3c-4.97 0-9 4.03-9 9H1l3.89 3.89.07.14L9 12H6c0-3.87 3.13-7 7-7s7 3.13 7 7-3.13 7-7 7c-1.93 0-3.68-.79-4.94-2.06l-1.42 1.42A8.954 8.954 0 0 0 13 21a9 9 0 0 0 0-18zm-1 5v5l4.28 2.54.72-1.21-3.5-2.08V8H12z"/>
      </svg>
      History
    </button>
    <button
      class="tab-button"
      class:active={activeTab === 'statistics'}
      on:click={() => activeTab = 'statistics'}
    >
      <svg viewBox="0 0 24 24" fill="currentColor" width="16" height="16">
        <path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V5h14v14zM7 10h2v7H7zm4-3h2v10h-2zm4 6h2v4h-2z"/>
      </svg>
      Statistics
    </button>
    <button
      class="tab-button"
      class:active={activeTab === 'exercise'}
      on:click={() => activeTab = 'exercise'}
    >
      <svg viewBox="0 0 24 24" fill="currentColor" width="16" height="16">
        <path d="M13.49 5.48c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm-3.6 13.9l1-4.4 2.1 2v6h2v-7.5l-2.1-2 .6-3c1.3 1.5 3.3 2.5 5.5 2.5v-2c-1.9 0-3.5-1-4.3-2.4l-1-1.6c-.4-.6-1-1-1.7-1-.3 0-.5.1-.8.1l-5.2 2.2v4.7h2v-3.4l1.8-.7-1.6 8.1-4.9-1-.4 2 7 1.4z"/>
      </svg>
      Exercise
    </button>
  </div>

  <!-- Tab Content -->
  {#if activeTab === 'history'}
    <!-- History Tab Content (D-03: preserve existing functionality) -->
    <div class="history-tab">
      <div class="history-header">
        <div class="header-left">
          <svg viewBox="0 0 24 24" fill="currentColor" width="24" height="24">
            <path d="M13 3c-4.97 0-9 4.03-9 9H1l3.89 3.89.07.14L9 12H6c0-3.87 3.13-7 7-7s7 3.13 7 7-3.13 7-7 7c-1.93 0-3.68-.79-4.94-2.06l-1.42 1.42A8.954 8.954 0 0 0 13 21a9 9 0 0 0 0-18zm-1 5v5l4.28 2.54.72-1.21-3.5-2.08V8H12z"/>
          </svg>
          <h3>Sessions</h3>
        </div>
        <div class="header-buttons">
          <button
            class="btn-refresh"
            on:click={() => { history.loadHistory(50); exercise.loadSessions(50); }}
          >
            <svg viewBox="0 0 24 24" fill="currentColor" width="14" height="14">
              <path d="M17.65 6.35A7.958 7.958 0 0 0 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08A5.99 5.99 0 0 1 12 18c-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/>
            </svg>
            Refresh
          </button>
          <button
            class="btn-export"
            on:click={() => showExportModal = true}
          >
            <svg viewBox="0 0 24 24" fill="currentColor" width="14" height="14">
              <path d="M19 9h-4V3H9v6H5l7 7 7-7zM5 18v2h14v-2H5z"/>
            </svg>
            Export
          </button>
        </div>
      </div>

      {#if $exercise.isLoading}
        <div class="loading-state">
          <div class="spinner"></div>
          <span>Loading sessions...</span>
        </div>
      {:else if $exercise.error}
        <div class="error-state">
          <svg viewBox="0 0 24 24" fill="currentColor" width="20" height="20">
            <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-2h2v2zm0-4h-2V7h2v6z"/>
          </svg>
          <span>Error: {$exercise.error}</span>
        </div>
      {:else if $exercise.sessions.length === 0}
        <div class="empty-state">
          <svg viewBox="0 0 24 24" fill="currentColor" width="48" height="48">
            <path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V5h14v14zM7 10h2v7H7zm4-3h2v10h-2zm4 6h2v4h-2z"/>
          </svg>
          <span>No sessions found</span>
          <span class="hint">Connect a device to start recording heart rate data</span>
        </div>
      {:else}
        <div class="sessions-list">
          {#each $exercise.sessions as session (session.session_id)}
            {@const detection = $exercise.detections.get(session.session_id)}
            <div class="session-row" class:expanded={expandedSession === session.session_id}>
              <div class="session-summary" on:click={() => toggleSession(session.session_id)}>
                <div class="session-info">
                  <svg class="expand-icon" viewBox="0 0 24 24" fill="currentColor" width="16" height="16">
                    <path d="M10 6L8.59 7.41 13.17 12l-4.58 4.59L10 18l6-6z"/>
                  </svg>
                  <span class="session-time">{formatSessionTime(session.start_time)}</span>
                  <span class="session-duration">{formatDurationMs(session.start_time, session.end_time)}</span>
                  <span class="session-records">{session.record_count} records</span>
                </div>
                <div class="session-metrics">
                  {#if session.exercise_tag}
                    <span class="exercise-badge">{session.exercise_tag.exercise_type}</span>
                  {/if}
                  <div class="record-bpm {getBpmClass(session.avg_bpm)}">
                    <span class="bpm-value">{Math.round(session.avg_bpm)}</span>
                    <span class="bpm-unit">avg</span>
                  </div>
                </div>
              </div>

              {#if expandedSession === session.session_id}
                <div class="session-details">
                  {#if !session.exercise_tag}
                    {#if detection && detection.is_exercise && detection.confidence >= 0.5}
                      <div class="detection-prompt">
                        <span class="confidence">Detected exercise ({Math.round(detection.confidence * 100)}% confidence)</span>
                        <span class="prompt-text">Was this exercise?</span>
                        <button class="btn-confirm" on:click={() => confirmDetection(session.session_id, detection)}>Yes</button>
                        <button class="btn-dismiss" on:click={() => dismissDetection(session.session_id)}>No</button>
                      </div>
                    {/if}

                    <div class="tagging-section">
                      <span class="tag-label">Tag as exercise:</span>
                      <div class="activity-pills">
                        {#each $exerciseTypes as type}
                          <button
                            class="pill"
                            class:active={selectedActivity === type}
                            on:click={() => selectedActivity = type}
                          >
                            {type}
                          </button>
                        {/each}
                      </div>
                      <div class="action-buttons">
                        <button class="btn-view" on:click={() => { selectedSessionId = session.session_id; showSessionChart = true; }}>
                          <svg viewBox="0 0 24 24" fill="currentColor" width="14" height="14">
                            <path d="M3 13h2v8H3v-8zm4-6h2v14H7V7zm4-4h2v18h-2V3zm4 8h2v10h-2V11zm4-3h2v13h-2V8z"/>
                          </svg>
                          View Chart
                        </button>
                        <button class="btn-tag" on:click={() => handleTagExercise(session.session_id)}>
                          Tag as Exercise
                        </button>
                        <button class="btn-delete" on:click={() => handleDeleteSession(session.session_id)}>
                          <svg viewBox="0 0 24 24" fill="currentColor" width="14" height="14">
                            <path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"/>
                          </svg>
                          Delete
                        </button>
                      </div>
                    </div>
                  {:else}
                    {#if editingTag === session.session_id}
                      <div class="edit-tag-section">
                        <span class="tag-label">Change exercise type:</span>
                        <div class="activity-pills">
                          {#each $exerciseTypes as type}
                            <button
                              class="pill"
                              class:active={editActivity === type}
                              on:click={() => editActivity = type}
                            >
                              {type}
                            </button>
                          {/each}
                        </div>
                        <div class="action-buttons">
                          <button class="btn-save" on:click={() => saveEditTag(session.session_id)}>
                            Save Changes
                          </button>
                          <button class="btn-cancel" on:click={cancelEditTag}>
                            Cancel
                          </button>
                        </div>
                      </div>
                    {:else if session.exercise_tag}
                      <div class="tagged-info">
                        <span class="tagged-label">Tagged as:</span>
                        <span class="tagged-type">{session.exercise_tag.exercise_type}</span>
                      </div>
                      <div class="action-buttons">
                        <button class="btn-view" on:click={() => { selectedSessionId = session.session_id; showSessionChart = true; }}>
                          <svg viewBox="0 0 24 24" fill="currentColor" width="14" height="14">
                            <path d="M3 13h2v8H3v-8zm4-6h2v14H7V7zm4-4h2v18h-2V3zm4 8h2v10h-2V11zm4-3h2v13h-2V8z"/>
                          </svg>
                          View Chart
                        </button>
                        <button class="btn-edit-tag" on:click={() => startEditTag(session.session_id, session.exercise_tag!.exercise_type)}>
                          <svg viewBox="0 0 24 24" fill="currentColor" width="14" height="14">
                            <path d="M3 17.25V21h3.75L17.81 9.94l-3.75-3.75L3 17.25zM20.71 7.04c.39-.39.39-1.02 0-1.41l-2.34-2.34c-.39-.39-1.02-.39-1.41 0l-1.83 1.83 3.75 3.75 1.83-1.83z"/>
                          </svg>
                          Edit Tag
                        </button>
                        <button class="btn-remove-tag" on:click={() => removeTag(session.session_id)}>
                          <svg viewBox="0 0 24 24" fill="currentColor" width="14" height="14">
                            <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/>
                          </svg>
                          Remove Tag
                        </button>
                        <button class="btn-delete" on:click={() => handleDeleteSession(session.session_id)}>
                          <svg viewBox="0 0 24 24" fill="currentColor" width="14" height="14">
                            <path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"/>
                          </svg>
                          Delete
                        </button>
                      </div>
                    {/if}
                  {/if}
                </div>
              {/if}
            </div>
          {/each}
        </div>
      {/if}
    </div>
  {:else if activeTab === 'exercise'}
    <!-- Exercise Tab Content -->
    <ExerciseTab />
  {:else}
    <!-- Statistics Tab Content -->
    <StatisticsTab />
  {/if}
</div>

{#if showExportModal}
  <ExportModal onClose={() => showExportModal = false} />
{/if}

{#if showSessionChart}
  <SessionChart sessionId={selectedSessionId} onClose={() => showSessionChart = false} />
{/if}

<style>
  .analytics-view {
    padding: 24px;
  }

  .tab-navigation {
    display: flex;
    gap: 8px;
    margin-bottom: 20px;
    padding-bottom: 16px;
    border-bottom: 1px solid var(--border-color);
  }

  .tab-button {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 10px 18px;
    background: transparent;
    color: var(--text-secondary);
    font-size: 14px;
    font-weight: 500;
    border: 1px solid transparent;
    border-radius: 10px;
    transition: all 0.2s ease;
  }

  .tab-button:hover {
    background: var(--bg-color);
    color: var(--text-primary);
  }

  .tab-button.active {
    background: var(--primary-color);
    color: white;
    border-color: var(--primary-color);
  }

  .tab-button svg {
    opacity: 0.8;
  }

  /* History tab styles */
  .history-tab {
    /* Content fills the space */
  }

  .history-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;
  }

  .header-left {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .header-left svg {
    color: var(--primary-color);
  }

  h3 {
    font-size: 18px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .btn-refresh {
    display: flex;
    align-items: center;
    gap: 6px;
    background: transparent;
    color: var(--text-secondary);
    font-size: 13px;
    padding: 8px 14px;
    border: 1px solid var(--border-color);
    border-radius: 8px;
    transition: all 0.2s ease;
  }

  .btn-refresh:hover {
    border-color: var(--primary-color);
    color: var(--primary-color);
  }

  .header-buttons {
    display: flex;
    gap: 8px;
  }

  .btn-export {
    display: flex;
    align-items: center;
    gap: 6px;
    background: transparent;
    color: var(--text-secondary);
    font-size: 13px;
    padding: 8px 14px;
    border: 1px solid var(--border-color);
    border-radius: 8px;
    transition: all 0.2s ease;
  }

  .btn-export:hover {
    border-color: var(--primary-color);
    color: var(--primary-color);
  }

  .loading-state,
  .error-state,
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 12px;
    padding: 48px;
    text-align: center;
    color: var(--text-muted);
    font-size: 14px;
  }

  .spinner {
    width: 24px;
    height: 24px;
    border: 2px solid var(--border-color);
    border-top-color: var(--primary-color);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .error-state {
    color: var(--danger-color);
  }

  .empty-state svg {
    opacity: 0.4;
  }

  .empty-state .hint {
    font-size: 12px;
    opacity: 0.7;
  }

  /* Sessions list styles */
  .sessions-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
    max-height: 400px;
    overflow-y: auto;
    padding-right: 4px;
  }

  .session-row {
    background: var(--bg-color);
    border-radius: 10px;
    transition: all 0.2s ease;
  }

  .session-row:hover {
    background: var(--card-bg-hover);
  }

  .session-summary {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 14px 16px;
    cursor: pointer;
  }

  .session-info {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .expand-icon {
    opacity: 0.5;
    transition: transform 0.2s ease;
  }

  .session-row.expanded .expand-icon {
    transform: rotate(90deg);
  }

  .session-time {
    font-size: 14px;
    font-weight: 500;
    color: var(--text-primary);
  }

  .session-duration,
  .session-records {
    font-size: 12px;
    color: var(--text-secondary);
  }

  .session-metrics {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .exercise-badge {
    font-size: 11px;
    font-weight: 500;
    padding: 4px 10px;
    background: var(--primary-light);
    color: var(--primary-color);
    border-radius: 12px;
  }

  .record-bpm {
    display: flex;
    align-items: baseline;
    gap: 4px;
    padding: 6px 12px;
    border-radius: 8px;
  }

  .record-bpm.low {
    background: var(--success-light);
  }

  .record-bpm.low .bpm-value {
    color: var(--success-color);
  }

  .record-bpm.normal {
    background: var(--primary-light);
  }

  .record-bpm.normal .bpm-value {
    color: var(--primary-color);
  }

  .record-bpm.elevated {
    background: var(--warning-light);
  }

  .record-bpm.elevated .bpm-value {
    color: var(--warning-color);
  }

  .record-bpm.high {
    background: var(--danger-light);
  }

  .record-bpm.high .bpm-value {
    color: var(--danger-color);
  }

  .bpm-value {
    font-size: 16px;
    font-weight: 700;
    font-variant-numeric: tabular-nums;
  }

  .bpm-unit {
    font-size: 11px;
    color: var(--text-muted);
  }

  /* Session details (expanded) */
  .session-details {
    padding: 16px;
    border-top: 1px solid var(--border-color);
    background: var(--bg-color);
    border-radius: 0 0 10px 10px;
  }

  .detection-prompt {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 16px;
    background: var(--warning-light);
    border-radius: 8px;
    margin-bottom: 12px;
  }

  .detection-prompt .confidence {
    font-size: 12px;
    color: var(--warning-color);
  }

  .detection-prompt .prompt-text {
    font-size: 13px;
    font-weight: 500;
    color: var(--text-primary);
  }

  .btn-confirm,
  .btn-dismiss {
    font-size: 12px;
    padding: 4px 12px;
    border-radius: 6px;
    border: none;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn-confirm {
    background: var(--primary-color);
    color: white;
  }

  .btn-confirm:hover {
    opacity: 0.9;
  }

  .btn-dismiss {
    background: var(--border-color);
    color: var(--text-secondary);
  }

  .btn-dismiss:hover {
    background: var(--text-muted);
    color: white;
  }

  .tagging-section {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .tag-label {
    font-size: 13px;
    color: var(--text-secondary);
  }

  .activity-pills {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
  }

  .pill {
    font-size: 12px;
    padding: 6px 14px;
    background: var(--bg-color);
    color: var(--text-secondary);
    border: 1px solid var(--border-color);
    border-radius: 16px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .pill:hover {
    border-color: var(--primary-color);
    color: var(--primary-color);
  }

  .pill.active {
    background: var(--primary-color);
    color: white;
    border-color: var(--primary-color);
  }

  .btn-tag {
    font-size: 13px;
    padding: 8px 16px;
    background: var(--primary-color);
    color: white;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn-tag:hover {
    opacity: 0.9;
  }

  .action-buttons {
    display: flex;
    gap: 8px;
    margin-top: 12px;
  }

  .btn-delete {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 13px;
    padding: 8px 16px;
    background: transparent;
    color: var(--danger-color);
    border: 1px solid var(--danger-color);
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn-delete:hover {
    background: var(--danger-color);
    color: white;
  }

  .btn-view {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 13px;
    padding: 8px 16px;
    background: var(--primary-color);
    color: white;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn-view:hover {
    opacity: 0.9;
  }

  .tagged-info {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .tagged-label {
    font-size: 13px;
    color: var(--text-secondary);
  }

  .tagged-type {
    font-size: 13px;
    font-weight: 500;
    color: var(--primary-color);
  }

  .edit-tag-section {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .btn-edit-tag {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 13px;
    padding: 8px 16px;
    background: transparent;
    color: var(--primary-color);
    border: 1px solid var(--primary-color);
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn-edit-tag:hover {
    background: var(--primary-color);
    color: white;
  }

  .btn-remove-tag {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 13px;
    padding: 8px 16px;
    background: transparent;
    color: var(--text-secondary);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn-remove-tag:hover {
    border-color: var(--text-secondary);
    color: var(--text-primary);
  }

  .btn-save {
    font-size: 13px;
    padding: 8px 16px;
    background: var(--primary-color);
    color: white;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn-save:hover {
    opacity: 0.9;
  }

  .btn-cancel {
    font-size: 13px;
    padding: 8px 16px;
    background: transparent;
    color: var(--text-secondary);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn-cancel:hover {
    border-color: var(--text-secondary);
    color: var(--text-primary);
  }
</style>