# HeartRate

[中文版](#中文版)

A Windows desktop application for receiving heart rate data from BLE devices.

## Features

- **BLE Device Scanning**: Scan and discover nearby heart rate monitors
- **Real-time Heart Rate Display**: View your heart rate in real-time with color-coded zones
- **Heart Rate Chart**: Visualize your heart rate history with an interactive chart
- **Fullscreen Mode**: Multiple display modes for immersive monitoring
- **Alert System**: Configurable high/low heart rate alerts with Windows notifications
- **History Tracking**: Local SQLite database for storing heart rate records
- **Session Management**: Record sessions with start/end times and heart rate data
- **Exercise Tagging**: Tag sessions as exercise with custom exercise types
- **Statistics**: View heart rate statistics by day/week/month
- **HRV Estimation**: Heart rate variability estimation from BPM variance
- **Exercise Analytics**: Compare exercise vs resting heart rate, view heart rate zones

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

---

# 中文版

一款用于从 BLE 设备接收心率数据的 Windows 桌面应用程序。

## 功能特性

- **BLE 设备扫描**：扫描并发现附近的心率监测设备
- **实时心率显示**：实时查看心率，带有颜色分区指示
- **心率曲线图**：交互式心率历史曲线可视化
- **全屏模式**：多种显示模式，沉浸式监测体验
- **警报系统**：可配置的高低心率警报，支持 Windows 通知
- **历史记录**：本地 SQLite 数据库存储心率记录
- **会话管理**：记录带有开始/结束时间和心率数据的会话
- **运动标记**：将会话标记为运动，支持自定义运动类型
- **统计分析**：按天/周/月查看心率统计
- **HRV 估算**：基于 BPM 方差的心率变异性估算
- **运动分析**：对比运动与静息心率，查看心率区间分布

## 技术栈

- **前端**：Svelte 5 + SvelteKit + TypeScript
- **后端**：Tauri 2.0 + Rust
- **蓝牙**：btleplug（跨平台蓝牙库）
- **数据库**：SQLite (rusqlite)
- **图表**：Chart.js

## 环境要求

- Node.js 18+
- Rust（最新稳定版）
- Windows 10/11 并启用蓝牙

## 开发

```bash
# 安装依赖
npm install

# 运行开发服务器
npm run tauri dev
```

## 构建

```bash
# 构建发布版本
npm run tauri build
```

## 支持的设备

任何实现标准蓝牙心率配置文件的 BLE 设备：
- 心率服务 UUID：`0x180D`
- 心率测量 UUID：`0x2A37`

测试过的设备：
- 华为手环系列
- 标准心率胸带
- 其他 BLE 心率监测器

## 许可证

MIT