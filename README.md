# Heart Rate Receiver

A Windows desktop application for receiving heart rate data from BLE devices.

## Features

- **BLE Device Scanning**: Scan and discover nearby heart rate monitors
- **Real-time Heart Rate Display**: View your heart rate in real-time with color-coded zones
- **Heart Rate Chart**: Visualize your heart rate history with an interactive chart
- **Fullscreen Mode**: Multiple display modes for immersive monitoring
- **Alert System**: Configurable high/low heart rate alerts with Windows notifications
- **History Tracking**: Local SQLite database for storing heart rate records

## Tech Stack

- **Frontend**: Svelte 5 + SvelteKit + TypeScript
- **Backend**: Tauri 2.0 + Rust
- **BLE**: btleplug (cross-platform Bluetooth library)
- **Database**: SQLite (rusqlite)
- **Charts**: Chart.js

## Prerequisites

- Node.js 18+
- Rust (latest stable)
- Windows 10/11 with Bluetooth enabled

## Development

```bash
# Install dependencies
npm install

# Run development server
npm run tauri dev
```

## Build

```bash
# Build release executable
npm run tauri build

# Build MSI installer
npm run tauri build -- --bundles msi
```

## Supported Devices

Any BLE device that implements the standard Bluetooth Heart Rate Profile:
- Heart Rate Service UUID: `0x180D`
- Heart Rate Measurement UUID: `0x2A37`

Tested with:
- Huawei Band series
- Standard heart rate chest straps
- Other BLE heart rate monitors

## License

MIT