import { writable } from 'svelte/store';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';

export interface DeviceInfo {
  id: string;
  name: string;
  rssi: number;
  supports_heart_rate: boolean;
}

export type ConnectionState =
  | 'Disconnected'
  | 'Connecting'
  | 'Connected'
  | 'Disconnecting'
  | { Error: string };

export interface DeviceState {
  isScanning: boolean;
  discoveredDevices: DeviceInfo[];
  connectedDevice: DeviceInfo | null;
  connectionState: ConnectionState;
}

function createDeviceStore() {
  const initialState: DeviceState = {
    isScanning: false,
    discoveredDevices: [],
    connectedDevice: null,
    connectionState: 'Disconnected',
  };

  const { subscribe, update, set } = writable<DeviceState>(initialState);
  let unlistenDevice: UnlistenFn | null = null;
  let unlistenConnection: UnlistenFn | null = null;
  // Use a Map for O(1) device lookups
  let deviceMap = new Map<string, DeviceInfo>();

  // Initialize listeners - call this in onMount
  async function initListeners() {
    if (typeof window === 'undefined') return;

    if (!unlistenDevice) {
      unlistenDevice = await listen<DeviceInfo>('device-discovered', (event) => {
        const device = event.payload;
        // O(1) lookup instead of O(n) findIndex
        const existing = deviceMap.get(device.id);
        if (existing) {
          // Update existing device
          deviceMap.set(device.id, device);
          update((state) => ({
            ...state,
            discoveredDevices: Array.from(deviceMap.values()),
          }));
        } else {
          // Add new device
          deviceMap.set(device.id, device);
          update((state) => ({
            ...state,
            discoveredDevices: [...state.discoveredDevices, device],
          }));
        }
      });
    }

    if (!unlistenConnection) {
      unlistenConnection = await listen<ConnectionState>('connection-state-changed', async (event) => {
        const newState = event.payload;
        update((state) => ({
          ...state,
          connectionState: newState,
        }));

        // Fetch connected device info when connected
        if (newState === 'Connected') {
          try {
            const device = await invoke<DeviceInfo | null>('get_connected_device');
            update((state) => ({
              ...state,
              connectedDevice: device,
            }));
          } catch (e) {
            console.error('Failed to get connected device:', e);
          }
        } else if (newState === 'Disconnected') {
          update((state) => ({
            ...state,
            connectedDevice: null,
          }));
        }
      });
    }
  }

  // Cleanup listeners
  function cleanup() {
    if (unlistenDevice) {
      unlistenDevice();
      unlistenDevice = null;
    }
    if (unlistenConnection) {
      unlistenConnection();
      unlistenConnection = null;
    }
  }

  return {
    subscribe,
    initListeners,
    cleanup,
    setScanning: (scanning: boolean) => {
      update((state) => ({
        ...state,
        isScanning: scanning,
      }));
    },
    setDiscoveredDevices: (devices: DeviceInfo[]) => {
      deviceMap.clear();
      devices.forEach(d => deviceMap.set(d.id, d));
      update((state) => ({
        ...state,
        discoveredDevices: devices,
      }));
    },
    setConnectedDevice: (device: DeviceInfo | null) => {
      update((state) => ({
        ...state,
        connectedDevice: device,
      }));
    },
    setConnectionState: (state: ConnectionState) => {
      update((s) => ({
        ...s,
        connectionState: state,
      }));
    },
    clearDevices: () => {
      deviceMap.clear();
      update((state) => ({
        ...state,
        discoveredDevices: [],
      }));
    },
    reset: () => {
      deviceMap.clear();
      set(initialState);
    },
    syncConnectionState: async () => {
      try {
        const state = await invoke<ConnectionState>('get_connection_state');
        const device = await invoke<DeviceInfo | null>('get_connected_device');
        update((s) => ({
          ...s,
          connectionState: state,
          connectedDevice: device,
        }));
      } catch (e) {
        console.error('Failed to sync connection state:', e);
      }
    },
  };
}

export const device = createDeviceStore();