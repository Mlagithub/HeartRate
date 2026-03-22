<script lang="ts">
  import '../app.css';
  import { onMount } from 'svelte';
  import { heartRate } from '$lib/stores/heartRate';
  import { device } from '$lib/stores/device';
  import { settings } from '$lib/stores/settings';

  // Initialize global event listeners at app level
  // This ensures they persist across navigation and tab switches
  onMount(() => {
    console.log('[Layout] onMount - initializing global listeners...');
    Promise.all([
      heartRate.initListener(),
      device.initListeners(),
      settings.loadSettings(),
    ]).then(() => {
      console.log('[Layout] All listeners initialized');
      device.syncConnectionState();
    });
  });
</script>

<slot />