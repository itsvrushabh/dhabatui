# DhabaTUI Mobile Client for Android 📱

A lightweight, offline-first read-only Android application designed for dhaba owners, kitchen cooks, and waitstaff to monitor live dining table orders and operations across private local Wi-Fi.

## ✨ Features
- **Read-Only Kitchen & Order Display**: View live order statuses (`Vacant`, `Ordering`, `Served`, `Billed`) for tables `T1`–`T8` and `Parcel`.
- **Itemized Breakdown**: Displays dish quantities, custom meat preparation notes (*Banwai*), and running table totals.
- **Local Network Wi-Fi Pairing**: Auto-polls the DhabaTUI counter's embedded `/api/status` endpoint (default port 8080).
- **Zero Cloud Dependency**: Communicates directly over local LAN Wi-Fi without needing internet access.
- **Offline Caching**: Retains last known dining room state with automatic reconnection heartbeats.

## 🛠️ Building & Installation
1. Open the `mobile/android` folder in **Android Studio (Hedgehog or newer)**.
2. Build APK:
   ```bash
   ./gradlew assembleDebug
   ```
3. Install onto any device running **Android 10+ (API 29+)**:
   ```bash
   adb install app/build/outputs/apk/debug/app-debug.apk
   ```
4. In the app, tap the **Settings (⚙)** icon to set your counter terminal's LAN IP address (displayed on the DhabaTUI counter screen, e.g. `192.168.1.50`).
