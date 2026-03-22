use crate::alert::notifier::{check_alert, reset_alert_cooldowns};
use crate::ble::{DeviceInfo, ConnectionState, HEART_RATE_SERVICE_UUID, HEART_RATE_MEASUREMENT_UUID};
use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use tokio::sync::RwLock;

/// BLE Manager state
pub struct BleManagerInner {
    /// Bluetooth adapter
    adapter: Option<Adapter>,
    /// Currently connected peripheral
    connected_peripheral: Option<Peripheral>,
    /// Discovered devices
    discovered_devices: HashMap<String, Peripheral>,
    /// Current connection state
    connection_state: ConnectionState,
    /// Connected device info
    connected_device: Option<DeviceInfo>,
    /// Scanning flag
    is_scanning: bool,
}

pub struct BleManager {
    inner: Arc<RwLock<BleManagerInner>>,
}

impl BleManager {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(BleManagerInner {
                adapter: None,
                connected_peripheral: None,
                discovered_devices: HashMap::new(),
                connection_state: ConnectionState::Disconnected,
                connected_device: None,
                is_scanning: false,
            })),
        }
    }

    /// Initialize the Bluetooth adapter
    async fn get_or_init_adapter(&self) -> Result<Adapter, String> {
        let mut inner = self.inner.write().await;

        if let Some(ref adapter) = inner.adapter {
            log::info!("Using cached Bluetooth adapter");
            return Ok(adapter.clone());
        }

        log::info!("Initializing Bluetooth adapter...");

        let manager = Manager::new()
            .await
            .map_err(|e| {
                log::error!("Failed to create Bluetooth manager: {}", e);
                format!("Failed to create Bluetooth manager: {}. Please ensure Bluetooth is enabled on your system.", e)
            })?;

        let adapters = manager
            .adapters()
            .await
            .map_err(|e| {
                log::error!("Failed to get adapters: {}", e);
                format!("Failed to get adapters: {}", e)
            })?;

        log::info!("Found {} Bluetooth adapter(s)", adapters.len());

        if adapters.is_empty() {
            return Err("No Bluetooth adapter found. Please ensure your computer has Bluetooth enabled.".to_string());
        }

        let adapter = adapters.into_iter().next()
            .ok_or_else(|| {
                log::error!("No Bluetooth adapter available");
                "No Bluetooth adapter available. Please ensure your computer has Bluetooth enabled.".to_string()
            })?;
        inner.adapter = Some(adapter.clone());

        log::info!("Bluetooth adapter initialized successfully");
        Ok(adapter)
    }

    /// Find a peripheral by device ID from the adapter
    async fn find_peripheral(&self, device_id: &str) -> Result<Peripheral, String> {
        let adapter = self.get_or_init_adapter().await?;

        // First try to get from cached devices
        {
            let inner = self.inner.read().await;
            if let Some(peripheral) = inner.discovered_devices.get(device_id) {
                log::info!("Found device in cache: {}", device_id);
                return Ok(peripheral.clone());
            }
        }

        // If not in cache, scan for it
        log::info!("Device not in cache, scanning for it...");

        // Start a quick scan
        adapter
            .start_scan(ScanFilter::default())
            .await
            .map_err(|e| format!("Failed to start scan: {}", e))?;

        // Wait for device to appear with exponential backoff
        let mut total_wait = 0u64;
        let delays = [100, 200, 300, 400, 500, 500, 500, 500, 500, 500,
                      1000, 1000, 1000, 1000, 1000, 2000, 2000, 2000, 2000, 2000];

        for delay in delays {
            tokio::time::sleep(Duration::from_millis(delay)).await;
            total_wait += delay;

            if let Ok(peripherals) = adapter.peripherals().await {
                for peripheral in peripherals {
                    if peripheral.id().to_string() == device_id {
                        log::info!("Found device via scan: {} (waited {}ms)", device_id, total_wait);
                        let _ = adapter.stop_scan().await;

                        // Add to cache
                        let mut inner = self.inner.write().await;
                        inner.discovered_devices.insert(device_id.to_string(), peripheral.clone());

                        return Ok(peripheral);
                    }
                }
            }
        }

        let _ = adapter.stop_scan().await;
        Err(format!("Device not found after {}ms: {}", total_wait, device_id))
    }

    /// Start scanning for heart rate devices
    pub async fn start_scan(&self, app_handle: AppHandle) -> Result<(), String> {
        log::info!("start_scan called");

        let adapter = self.get_or_init_adapter().await?;

        // Check if already scanning
        {
            let inner = self.inner.read().await;
            if inner.is_scanning {
                log::warn!("Already scanning, returning error");
                return Err("Already scanning".to_string());
            }
        }

        // Clear previous discoveries and set scanning flag
        {
            let mut inner = self.inner.write().await;
            inner.discovered_devices.clear();
            inner.is_scanning = true;
        }

        log::info!("Starting BLE scan...");

        // Start scanning with empty filter to discover all devices
        match adapter.start_scan(ScanFilter::default()).await {
            Ok(_) => log::info!("Scan started successfully"),
            Err(e) => {
                log::error!("Failed to start scan: {}", e);
                let mut inner = self.inner.write().await;
                inner.is_scanning = false;
                return Err(format!("Failed to start scan: {}. Please check if Bluetooth is enabled and app has permission.", e));
            }
        }

        // Spawn a task to continuously discover devices
        let inner_clone = self.inner.clone();
        let app_handle_clone = app_handle.clone();

        tokio::spawn(async move {
            log::info!("Scan task started, waiting for devices...");

            // Scan for 30 seconds
            let scan_duration = Duration::from_secs(30);
            let start_time = std::time::Instant::now();
            let mut check_count = 0;

            while start_time.elapsed() < scan_duration {
                // Check if we should stop
                {
                    let inner = inner_clone.read().await;
                    if !inner.is_scanning {
                        log::info!("Scan stopped by user");
                        break;
                    }
                }

                // Sleep a bit then check peripherals
                tokio::time::sleep(Duration::from_millis(500)).await;
                check_count += 1;

                // Get all peripherals
                match adapter.peripherals().await {
                    Ok(peripherals) => {
                        if check_count % 4 == 0 {
                            log::info!("Check #{}: Found {} peripheral(s)", check_count, peripherals.len());
                        }

                        for peripheral in peripherals {
                            match peripheral.properties().await {
                                Ok(Some(props)) => {
                                    let name = props.local_name.clone().unwrap_or_else(|| "Unknown".to_string());
                                    let rssi = props.rssi.unwrap_or(-100);
                                    let device_id = peripheral.id().to_string();

                                    // Check if device supports heart rate service
                                    let supports_hr = props
                                        .services
                                        .iter()
                                        .any(|s| *s == HEART_RATE_SERVICE_UUID);

                                    // Single read to check if device already exists
                                    let is_new_device = {
                                        let inner = inner_clone.read().await;
                                        !inner.discovered_devices.contains_key(&device_id)
                                    };

                                    if is_new_device {
                                        log::info!("NEW DEVICE: '{}' (RSSI: {}, HR: {})", name, rssi, supports_hr);

                                        let device_info = DeviceInfo {
                                            id: device_id.clone(),
                                            name: name.clone(),
                                            rssi,
                                            supports_heart_rate: supports_hr,
                                        };

                                        {
                                            let mut inner = inner_clone.write().await;
                                            inner.discovered_devices.insert(device_id, peripheral);
                                        }

                                        // Emit device discovered event
                                        if let Err(e) = app_handle_clone.emit("device-discovered", &device_info) {
                                            log::error!("Failed to emit device event: {}", e);
                                        }
                                    } else if name != "Unknown" {
                                        // Update existing device with real name
                                        let device_info = DeviceInfo {
                                            id: device_id.clone(),
                                            name,
                                            rssi,
                                            supports_heart_rate: supports_hr,
                                        };

                                        // Emit update event
                                        if let Err(e) = app_handle_clone.emit("device-discovered", &device_info) {
                                            log::error!("Failed to emit device update event: {}", e);
                                        }
                                    }
                                }
                                Ok(None) => {
                                    log::debug!("Peripheral has no properties");
                                }
                                Err(e) => {
                                    log::debug!("Failed to get peripheral properties: {}", e);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to get peripherals: {}", e);
                    }
                }
            }

            // DON'T stop scanning here - let user stop it or connect
            // Only update is_scanning flag
            let mut inner = inner_clone.write().await;
            inner.is_scanning = false;

            log::info!("Scan task completed. Total devices found: {}", inner.discovered_devices.len());
        });

        Ok(())
    }

    /// Stop scanning
    pub async fn stop_scan(&self) -> Result<(), String> {
        log::info!("stop_scan called");

        let adapter = self.get_or_init_adapter().await?;

        {
            let mut inner = self.inner.write().await;
            inner.is_scanning = false;
        }

        adapter
            .stop_scan()
            .await
            .map_err(|e| format!("Failed to stop scan: {}", e))
    }

    /// Get discovered devices
    pub async fn get_devices(&self) -> Vec<DeviceInfo> {
        let inner = self.inner.read().await;
        let mut devices: Vec<DeviceInfo> = Vec::new();

        for (id, peripheral) in &inner.discovered_devices {
            if let Ok(Some(properties)) = peripheral.properties().await {
                let name = properties.local_name.clone().unwrap_or_else(|| "Unknown".to_string());
                let rssi = properties.rssi.unwrap_or(-100);
                let supports_hr = properties
                    .services
                    .iter()
                    .any(|s| *s == HEART_RATE_SERVICE_UUID);

                devices.push(DeviceInfo {
                    id: id.clone(),
                    name,
                    rssi,
                    supports_heart_rate: supports_hr,
                });
            }
        }

        devices.sort_by(|a, b| b.rssi.cmp(&a.rssi));
        devices
    }

    /// Connect to a specific device
    pub async fn connect_device(&self, device_id: String, app_handle: AppHandle) -> Result<(), String> {
        log::info!("connect_device called for: {}", device_id);

        // Stop any ongoing scan first
        {
            let inner = self.inner.read().await;
            if inner.is_scanning {
                drop(inner);
                let adapter = self.get_or_init_adapter().await?;
                let _ = adapter.stop_scan().await;
                let mut inner = self.inner.write().await;
                inner.is_scanning = false;
            }
        }

        // Find the peripheral
        let peripheral = self.find_peripheral(&device_id).await?;

        // Update connection state
        {
            let mut inner = self.inner.write().await;
            inner.connection_state = ConnectionState::Connecting;
            let _ = app_handle.emit("connection-state-changed", &ConnectionState::Connecting);
        }

        log::info!("Connecting to device...");

        // Connect to the peripheral
        let connect_result = peripheral.connect().await;
        if let Err(e) = connect_result {
            log::error!("Failed to connect: {}", e);
            let mut inner = self.inner.write().await;
            inner.connection_state = ConnectionState::Error(e.to_string());
            let _ = app_handle.emit("connection-state-changed", &inner.connection_state);
            return Err(format!("Failed to connect: {}", e));
        }

        log::info!("Connected, discovering services...");

        // Discover services
        peripheral
            .discover_services()
            .await
            .map_err(|e| format!("Failed to discover services: {}", e))?;

        // Log all characteristics for debugging
        let characteristics = peripheral.characteristics();
        log::info!("Found {} characteristics", characteristics.len());
        for c in &characteristics {
            log::info!("Characteristic: {} (properties: {:?})", c.uuid, c.properties);
        }

        // Find heart rate service and characteristic
        let hr_characteristic = characteristics
            .iter()
            .find(|c| c.uuid == HEART_RATE_MEASUREMENT_UUID)
            .ok_or_else(|| {
                log::error!("Heart Rate Measurement characteristic not found");
                "Heart Rate Measurement characteristic not found. This device may not support standard Heart Rate Profile.".to_string()
            })?;

        // Subscribe to notifications
        let inner_clone = self.inner.clone();
        let app_handle_clone = app_handle.clone();

        peripheral
            .subscribe(hr_characteristic)
            .await
            .map_err(|e| {
                log::error!("Failed to subscribe: {}", e);
                format!("Failed to subscribe to heart rate notifications: {}", e)
            })?;

        log::info!("Subscribed to heart rate notifications");

        // Get notification stream BEFORE spawning the task
        let notification_stream = peripheral.notifications().await
            .map_err(|e| {
                log::error!("Failed to get notification stream: {}", e);
                format!("Failed to get notification stream: {}", e)
            })?;

        log::info!("Notification stream created successfully");

        // Update state
        {
            let mut inner = self.inner.write().await;
            inner.connection_state = ConnectionState::Connected;
            inner.connected_peripheral = Some(peripheral.clone());

            if let Ok(Some(properties)) = peripheral.properties().await {
                inner.connected_device = Some(DeviceInfo {
                    id: peripheral.id().to_string(),
                    name: properties.local_name.clone().unwrap_or_else(|| "Unknown".to_string()),
                    rssi: properties.rssi.unwrap_or(-100),
                    supports_heart_rate: true,
                });
            }

            let _ = app_handle.emit("connection-state-changed", &ConnectionState::Connected);
        }

        // Spawn notification handler
        tokio::spawn(async move {
            log::info!("Listening for heart rate notifications...");
            use futures::stream::StreamExt;

            let mut notification_stream = notification_stream;
            let mut heartbeat_counter = 0u32;

            loop {
                // Use tokio::select with a timeout to detect if stream is stalled
                tokio::select! {
                    notification_opt = notification_stream.next() => {
                        match notification_opt {
                            Some(notification) => {
                                if notification.uuid == HEART_RATE_MEASUREMENT_UUID {
                                    if let Some(measurement) = crate::ble::heart_rate::parse_heart_rate_measurement(&notification.value) {
                                        log::debug!("Received heart rate: {} BPM", measurement.bpm);
                                        if let Err(e) = app_handle_clone.emit("heart-rate-measurement", &measurement) {
                                            log::error!("Failed to emit heart-rate-measurement event: {}", e);
                                        }
                                        let _ = check_alert(measurement.bpm, &app_handle_clone).await;
                                    }
                                }
                            }
                            None => {
                                log::warn!("Notification stream ended (None received)");
                                break;
                            }
                        }
                    }
                    _ = tokio::time::sleep(std::time::Duration::from_secs(5)) => {
                        heartbeat_counter += 1;
                        log::debug!("Heartbeat {}: waiting for notifications...", heartbeat_counter);
                    }
                }
            }

            log::warn!("Notification stream ended normally");
            let mut inner = inner_clone.write().await;
            inner.connection_state = ConnectionState::Disconnected;
            inner.connected_peripheral = None;
            inner.connected_device = None;
            let _ = app_handle_clone.emit("connection-state-changed", &ConnectionState::Disconnected);
        });

        Ok(())
    }

    /// Disconnect from current device
    pub async fn disconnect_device(&self, app_handle: AppHandle) -> Result<(), String> {
        let peripheral = {
            let mut inner = self.inner.write().await;
            inner.connection_state = ConnectionState::Disconnecting;
            let _ = app_handle.emit("connection-state-changed", &ConnectionState::Disconnecting);
            inner.connected_peripheral.clone()
        };

        if let Some(peripheral) = peripheral {
            peripheral
                .disconnect()
                .await
                .map_err(|e| format!("Failed to disconnect: {}", e))?;
        }

        // Reset alert cooldowns on disconnect
        reset_alert_cooldowns();

        let mut inner = self.inner.write().await;
        inner.connection_state = ConnectionState::Disconnected;
        inner.connected_peripheral = None;
        inner.connected_device = None;
        let _ = app_handle.emit("connection-state-changed", &ConnectionState::Disconnected);

        Ok(())
    }

    /// Get current connection state
    pub async fn get_connection_state(&self) -> ConnectionState {
        self.inner.read().await.connection_state.clone()
    }

    /// Get connected device info
    pub async fn get_connected_device(&self) -> Option<DeviceInfo> {
        self.inner.read().await.connected_device.clone()
    }
}

impl Default for BleManager {
    fn default() -> Self {
        Self::new()
    }
}