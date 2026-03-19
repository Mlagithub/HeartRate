import { invoke } from '@tauri-apps/api/core';
import type { DeviceInfo, ConnectionState } from './stores/device';
import type { HeartRateMeasurement } from './stores/heartRate';
import type { AlertSettings, HeartRateRecord } from './stores/settings';

// BLE Commands
export async function startScan(): Promise<void> {
  return invoke('start_scan');
}

export async function stopScan(): Promise<void> {
  return invoke('stop_scan');
}

export async function getDevices(): Promise<DeviceInfo[]> {
  return invoke('get_devices');
}

export async function connectDevice(deviceId: string): Promise<void> {
  return invoke('connect_device', { deviceId });
}

export async function disconnectDevice(): Promise<void> {
  return invoke('disconnect_device');
}

export async function getConnectionState(): Promise<ConnectionState> {
  return invoke('get_connection_state');
}

// Storage Commands
export async function getHeartRateHistory(limit: number = 100, offset: number = 0): Promise<HeartRateRecord[]> {
  return invoke('get_heart_rate_history', { limit, offset });
}

export async function saveHeartRate(measurement: HeartRateMeasurement, sessionId: string): Promise<number> {
  return invoke('save_heart_rate', { measurement, sessionId });
}

// Settings Commands
export async function setAlertSettings(settings: AlertSettings): Promise<void> {
  return invoke('set_alert_settings', { settings });
}

export async function getAlertSettings(): Promise<AlertSettings> {
  return invoke('get_alert_settings');
}

// Database Commands
export async function initDatabase(): Promise<void> {
  return invoke('init_database');
}