# GitHub Copilot Instructions for DhabaTUI

Welcome to the **DhabaTUI** codebase! DhabaTUI is a lightning-fast, keyboard-driven Terminal User Interface (TUI) Point-of-Sale (POS) and billing application built specifically for Indian roadside dhabas, family-run eateries, and small restaurants.

When generating code, analyzing pull requests, or providing review comments for this repository, you **MUST** strictly follow the guidelines, architectural patterns, and constraints described below.

---

## 1. Core Principles & Philosophy

- **Zero-Mouse POS Speed**: Dhaba cashiers must be able to operate the entire application at peak rush hours using only their keyboard. Never require mouse clicks for essential billing tasks.
- **Resilience Against Power Loss**: Highway dhabas frequently experience sudden power outages. Every transaction must be atomically committed to disk via ACID storage before presenting confirmation.
- **Minimal Dependencies & Fast Compilation**: Avoid heavy runtime frameworks (e.g. no async Tokio runtimes or heavy ORMs). The application must remain a lightweight, self-contained binary.
- **Distraction-Free Terminal**: The primary cashier screen must remain clean and uncluttered. Do not expose internal network URLs or debugging text on the main terminal screen.

---

## 2. Technical Stack

| Layer | Library / Tool | Constraints |
| :--- | :--- | :--- |
| **Language** | Rust (2021 Edition) | Adhere to standard Rust idioms and zero-cost abstractions. |
| **Terminal UI** | `ratatui` (v0.29+) + `crossterm` (v0.28+) | Always use alternate screen buffers and manage raw terminal mode safely. |
| **ACID Database** | `redb` (v2.x) | Single-file embedded storage (`dhabatui.redb`). No external DB engines. |
| **Web Server** | `tiny_http` (v0.12+) | Synchronous background thread. Zero async runtime overhead. |
| **Serialization** | `serde` + `serde_json` | Clean JSON models for state synchronization. |
| **Logging** | `tracing` + `tracing-appender` | Non-blocking file logging to `dhabatui.log`. Never print logs directly to stdout/stderr. |

---

## 3. Strict Rules & Architectural Constraints

### ⚠️ A. The "Zero Floating-Point for Money" Rule (MANDATORY)
- **NEVER use `f32` or `f64` for currency, pricing, taxes, or bill totals.** Floating-point inaccuracies lead to accounting errors and cashier distrust.
- **Always store and calculate money in integer paise (`u32` or `u64`)**:
  - `1 Rupee = 100 paise` (e.g., ₹130.00 is stored as `13000`).
- **Formatting**: Always format paise for display using `crate::models::format_paise(paise: u32) -> String` (outputs `"₹130.00"`).
- **Taxes**: Dhaba standard GST is 5% total split into:
  - **CGST (2.5%)**: `((subtotal_paise * 25) + 500) / 1000` (exact integer half-up rounding).
  - **SGST (2.5%)**: `((subtotal_paise * 25) + 500) / 1000` (exact integer half-up rounding).
  - `grand_total = subtotal + cgst + sgst`.

### ⚠️ B. Multi-Table Management
- Support 8 dining tables (`T1` through `T8`) and 1 `Parcel Takeaway` order simultaneously.
- Tables have operational states: `Vacant`, `Ordering`, `Served`, `Billed`.
- Adding an item to a `Vacant` table automatically transitions it to `Ordering`.
- Carts must remain strictly isolated between tables.

### ⚠️ C. Customer-Supplied Meat / Making Charges (*Banwai*)
- Highway dhabas frequently cook raw meat or chicken brought in by customers.
- Support custom dish descriptions and negotiated cooking fees via the Making Charge flow (<kbd>m</kbd>).
- Custom items must be marked with `is_custom: true` and prefixed with `*` on printed receipts and web cards.
- Support on-the-go price alteration (<kbd>e</kbd>) without altering permanent menu database records.

### ⚠️ D. Embedded Web Dashboard & LAN-Only Security
- The embedded web server runs in a background thread serving a live read-only dashboard for kitchen monitors and waitstaff phones.
- **LAN-Only Protection**: The server must enforce RFC 1918 and loopback address validation on every request via `is_lan_ip(&remote_addr)`.
  - Allowed: `127.0.0.1`, `::1`, `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`, `169.254.0.0/16`.
  - Forbidden: Any external/public IP address must receive HTTP **403 Forbidden**.
- **State Synchronization**: Application state is shared using thread-safe `Arc<RwLock<DashboardData>>`. Minimize write-lock hold times.
- **Do not introduce Tokio or async frameworks**: Keep `tiny_http` synchronous.

### ⚠️ E. Terminal Safety & Panic Recovery
- Ensure alternate screen mode (`EnterAlternateScreen` / `LeaveAlternateScreen`) and raw mode (`enable_raw_mode()` / `disable_raw_mode()`) are properly paired.
- Keep the custom panic hook intact in `main.rs` to guarantee that terminal settings are restored even if the application panics.

### ⚠️ F. Documentation & CLI Command Rules
- **NEVER** instruct the user to run `cargo run --release` in user documentation, READMEs, or instructions. Always use the installed binaries:
  ```bash
  dti
  # or
  dhabatui
  ```
- Keep the root `README.md` clean, cashier-friendly, and concise. Move all deep architectural, financial, and operating guides into `docs/`.

---

## 4. Banned Patterns & Dependencies

| Banned Pattern | Preferred Alternative | Reason |
| :--- | :--- | :--- |
| `f32` / `f64` for money | `u32` / `u64` paise | Prevents floating-point rounding errors. |
| `tokio` / `axum` / `actix-web` | Synchronous `tiny_http` background thread | Avoids massive async runtime bloat and slow compiles. |
| `println!` / `eprintln!` inside TUI loop | `tracing::info!` to `dhabatui.log` | Prevents terminal screen corruption and escape code breakage. |
| Unrestricted `0.0.0.0` exposure without IP check | RFC 1918 check (`is_lan_ip`) | Prevents exposing live dhaba billing data to the public internet. |
| Mouse-only interactions | Keyboard hotkeys (<kbd>t</kbd>, <kbd>s</kbd>, <kbd>m</kbd>, <kbd>e</kbd>, <kbd>p</kbd>, etc.) | Cashiers require maximum keyboard speed. |
| External database servers (Postgres/MySQL) | Embedded `redb` (`dhabatui.redb`) | Zero-ops embedded database; survives abrupt power loss. |

---

## 5. Code Review & Verification Checklist for Copilot

When reviewing pull requests or generating modifications, ensure:
1. **Formatting**: Code adheres to `cargo fmt --all`.
2. **Linting**: Code passes `cargo clippy --all-targets -- -D warnings` with zero warnings.
3. **Tests**: All existing unit and integration tests pass (`cargo test --all-targets`). Add unit tests for any new state logic or arithmetic.
4. **Receipt Accuracy**: Ensure printed receipts format in standard 80mm column width and match integer paise totals.
5. **Documentation**: Update relevant files in `docs/` if modifying state, schema, or workflows.
