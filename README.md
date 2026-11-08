# 🍲 DhabaTUI

[![CI](https://github.com/itsvrushabh/dhabatui/actions/workflows/ci.yml/badge.svg)](https://github.com/itsvrushabh/dhabatui/actions/workflows/ci.yml)
[![Website](https://img.shields.io/badge/Website-itsvrushabh.github.io%2Fdhabatui-amber?logo=googlechrome)](https://itsvrushabh.github.io/dhabatui/)
[![Release](https://img.shields.io/github/v/release/itsvrushabh/dhabatui?color=blue&label=Release)](https://github.com/itsvrushabh/dhabatui/releases)
[![License](https://img.shields.io/badge/License-MIT-green)](LICENSE)

**Fast, 100% offline POS and billing software designed specifically for Indian dhabas, roadside eateries, tea stalls, and casual restaurants.**

👉 **[Visit the Official Website & User Guide](https://itsvrushabh.github.io/dhabatui/)**

---

## 💡 What is DhabaTUI? (In Plain English)

DhabaTUI is a lightweight cash register and billing application built for busy counter staff. It runs completely offline on any regular computer or laptop — meaning **you never lose a bill during power cuts or internet outages**.

Instead of struggling with a slow mouse or complex cloud software during lunch and dinner rush hours, cashiers can punch a full table's order and settle an itemized bill (saved to `receipt.txt`) in **under 5 seconds** using simple keyboard shortcuts.

---

## 🌟 Why Dhaba Owners Love It

- 🔌 **100% Offline**: No internet or cloud subscription needed. Everything runs locally on your counter computer.
- 💰 **Exact Paise Accuracy**: Uses zero-float integer math (`100 paise = ₹1.00`). Eliminates computer rounding drift so your cash drawer balances to the single rupee every night.
- ⚡ **Ultra-Fast Keyboard Entry**: Memorize dish codes (like `101` for Dal Tadka) and add items in milliseconds.
- 🪑 **8 Tables + Parcel Orders**: Switch between Dining Tables (`T1`–`T8`) and takeaway parcels with the <kbd>t</kbd> key. Focus menu/cart with <kbd>Tab</kbd>.
- 🥩 **Customer Meat Cooking (*Banwai*)**: Easily add custom meat preparation charges on the fly when guests bring their own raw chicken or mutton.
- 📱 **Mobile View for Waiters & Kitchen**: Kitchen staff and waiters can open their phone browser on your dhaba's local Wi-Fi to view active table orders in real-time.

---

## 🖥️ What It Looks Like

![DhabaTUI Screenshot](assets/screenshot.svg)

---

## 🚀 Get Started in 3 Steps (No Coding Required)

### 1. Download for Your Computer
Download the ready-to-run package from our **[Latest Release](https://github.com/itsvrushabh/dhabatui/releases/latest)**:

* 🪟 **Windows**: [Download `.zip`](https://github.com/itsvrushabh/dhabatui/releases/latest) (Works on Windows 10 & 11)
* 🍏 **Mac (Apple Silicon M1/M2/M3/M4)**: [Download `.tar.gz`](https://github.com/itsvrushabh/dhabatui/releases/latest) *(Note: On first launch, right-click and choose "Open" if prompted by Gatekeeper)*
* 🍏 **Mac (Intel)**: [Download `.tar.gz`](https://github.com/itsvrushabh/dhabatui/releases/latest) *(Note: On first launch, right-click and choose "Open" if prompted by Gatekeeper)*
* 🐧 **Linux**: [Download `.tar.gz`](https://github.com/itsvrushabh/dhabatui/releases/latest)

### 2. Extract the Folder
Right-click the downloaded file and select **Extract All** (or uncompress).

### 3. Open and Start Billing!
* **Windows**: Double-click `dti.exe`
* **Mac & Linux**: Open terminal and run `./dti`

*(Are you a Rust developer? You can also install via Cargo: `cargo install dhabatui`)*

---

## ⌨️ Common Cashier Shortcuts

| Key | What It Does | When to Use |
| :---: | :--- | :--- |
| <kbd>t</kbd> / <kbd>T</kbd> | Cycle Tables (`T1` to `T8`, Parcel) | When guests sit at a new table |
| <kbd>s</kbd> | Change Table Status | Cycle status: `Vacant` ➔ `Ordering` ➔ `Served` ➔ `Billed` ➔ `Ordering` (resets on settle <kbd>p</kbd> or clear <kbd>c</kbd>) |
| <kbd>Tab</kbd> | Switch Focus (Menu ⟷ Cart) | To browse dishes or adjust order lines |
| <kbd>/</kbd> | Search Dishes in Real-Time | Type dish name or code to filter menu |
| <kbd>f</kbd> / <kbd>F</kbd> | Filter by Category | Cycle Mains ➔ Breads ➔ Rice ➔ Making ➔ Drinks |
| <kbd>0</kbd>–<kbd>9</kbd> + <kbd>Enter</kbd> | Quick Dish Number (e.g. `101`) | To punch items in 1 second |
| <kbd>+</kbd> / <kbd>-</kbd> | Increase / Decrease item count | When guests order extra rotis |
| <kbd>m</kbd> | Customer Cooking Fee (*Banwai*) | For customer-brought raw meat preparation |
| <kbd>e</kbd> | Change Price On-the-Fly | To adjust price or give discount |
| <kbd>p</kbd> | Settle Bill & Save Receipt | Finalize order and save to `receipt.txt` |
| <kbd>q</kbd> / <kbd>Esc</kbd> | Close Window / Exit | To close any popup safely |

---

## 📚 Complete Guides & Documentation

* 🌐 **[Official Website](https://itsvrushabh.github.io/dhabatui/)**: Visual guides and downloads.
* 📖 **[Project Wiki](https://github.com/itsvrushabh/dhabatui/wiki)** (or local [`wiki/`](wiki/)): Full knowledge base and operations manual.
* 👨‍🍳 **[Cashier's Visual Guide](docs/CASHIER_GUIDE.md)**: Visual 4-step workflow for counter staff.
* 📱 **[LAN Mobile Dashboard](docs/WEB_DASHBOARD.md)**: How to view orders on mobile phones over local Wi-Fi.
* 💰 **[Financials & Zero-Float Invariant](docs/FINANCIALS.md)**: Exact paise math and half-up GST rounding.
* 🗓️ **[Release Cycle & Cadence](docs/RELEASE_CYCLE.md)**: Yearly LTS, Monthly Minor, and Weekly Patch schedules.
* 📜 **[Changelog](CHANGELOG.md)**: Version history and updates.
* 🛠️ **[Developer & Contribution Guide](docs/DEVELOPMENT.md)**: Compiling, testing, and architecture.

---

## 📄 License

DhabaTUI is free and open-source software licensed under the [MIT License](LICENSE).
