# DhabaTUI Mobile Client for iOS & iPadOS 🍏

A lightweight, offline-first read-only iOS application built with SwiftUI for iPhone and iPad. Enables dhaba owners, cooks, and waitstaff to monitor live dining table orders and operational revenue metrics over private local Wi-Fi.

## ✨ Features
- **Read-Only Kitchen & Order Display**: View live dining table cards (`T1`–`T8` and `Parcel`) with operational status badges (`Vacant`, `Ordering`, `Served`, `Billed`).
- **Real-Time Auto-Polling**: Connects to the counter's embedded `/api/status` endpoint every 2.5 seconds.
- **Local Network Support**: Configured with `NSLocalNetworkUsageDescription` in `Info.plist` for local Wi-Fi discovery.
- **Offline Caching**: Automatically preserves last fetched dining orders with connection heartbeat indicator.
- **Responsive Layout**: Adapts gracefully across iPhone (portrait) and iPad (multi-column landscape).

## 🛠️ Building & Installation
1. Open the `mobile/ios` project in **Xcode 15+**.
2. Target: **iOS 15.0+** or **iPadOS 15.0+**.
3. Select your iPhone / iPad or simulator and click **Run (Cmd + R)**.
4. On first launch, allow the iOS Local Network permission prompt.
5. Tap the **Settings (⚙)** icon to enter your counter terminal's LAN IP address (e.g., `192.168.1.50`).
