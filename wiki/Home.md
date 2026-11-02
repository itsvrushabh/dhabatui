# DhabaTUI Wiki

Welcome to the official documentation and knowledge base for **DhabaTUI** — the fast, keyboard-first Terminal POS and billing application designed for Indian dhabas, tea stalls, food trucks, and casual eateries.

---

## 📖 Navigation & Sitemap

| Section | Description |
| :--- | :--- |
| **[Release Cycle & Cadence](Release-Cycle-and-Cadence)** | Yearly LTS, Monthly Minor, and Weekly Patch cadence, support matrix, and SemVer rules |
| **[Release Automation Guide](Release-Automation-Guide)** | GitHub Actions release workflows, automated tag generation, and crates.io publishing |
| **[Architecture & Financials](Architecture-and-Financials)** | Integer paise math guarantee, Redb ACID persistence, LAN HTTP server, and read-only mobile dashboard |
| **[Developer & Contribution Guide](Developer-and-Contribution-Guide)** | Local setup, building, testing, linting conventions, and pull request review workflows |
| **[Cashier & Staff Operations](Cashier-and-Staff-Operations)** | Rapid keyboard shortcuts, table billing, customer meat cooking (Banwai), and receipt generation |

---

## 🎯 What is DhabaTUI?

DhabaTUI is engineered to survive real-world dhaba conditions:
- **Zero Internet Requirement**: Operates 100% offline with zero cloud dependency.
- **Flawless Financial Accounting**: Absolute integer arithmetic in paise (`100 paise = ₹1.00`) with half-up GST rounding. Eliminates IEEE 754 floating-point drift.
- **Ultra-Fast Keyboard Navigation**: Cashiers can punch orders, add custom cooking charges (*banwai*), and settle bills in under 5 seconds using single-key shortcuts.
- **Embedded Local LAN Server**: Runs a lightweight, zero-latency local HTTP server (`tiny_http`) for mobile viewing across dining halls over local Wi-Fi.
- **Multi-Platform Native Binaries**: Pre-compiled zero-dependency binaries for Linux (x86_64), macOS (Apple Silicon arm64 & Intel x86_64), and Windows (x86_64).

---

## 🚀 Quickstart

### Installation via Cargo
```bash
cargo install dhabatui
```

### Launching the App (`dti` / `dhabatui`)
DhabaTUI provides both `dhabatui` and the rapid 3-letter alias `dti`:
```bash
dti
# or
dhabatui
```
Both launch the full terminal POS counter interface instantly with zero startup delay.

---

## 🏷️ Release Channels

DhabaTUI adheres to strict, automated release cycles:
* **Yearly LTS (`vX.0.0`)**: 12 months stability, critical security patches only.
* **Monthly Feature (`vX.Y.0`)**: 1st of every month with new enhancements.
* **Weekly Patch (`vX.Y.Z`)**: Every Sunday with bug fixes and optimizations.

For complete release details, see the **[Release Cycle & Cadence](Release-Cycle-and-Cadence)** guide.
