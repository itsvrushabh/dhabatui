# DhabaTUI - Specification & Implementation Plan

`dhabatui` is a fast, keyboard-first Terminal User Interface (TUI) Point-of-Sale (POS) and billing application designed for dhabas and casual restaurants. It is written in idiomatic Rust using `ratatui` (0.29+), `crossterm` (0.28+), and `rusqlite` (with bundled SQLite).

---

### Key Requirements

1. **Modular Architecture:**
   - `Cargo.toml`: All required dependencies and metadata.
   - `src/models.rs`: Domain models (`MenuItem`, `OrderItem`, `BillSummary`).
   - `src/db.rs`: SQLite initialization, default menu seeding, and order persistence.
   - `src/app.rs`: Application state machine, key action handlers, and business logic.
   - `src/ui.rs`: Pure rendering logic using Ratatui layouts, tables, lists, and styled spans.
   - `src/main.rs`: Terminal alternate screen/raw mode lifecycle, custom panic hook for terminal cleanup, and event loop.

2. **Core Functionalities:**
   - **Menu Catalog (Left Pane):** Scrollable list with item codes (`[101] Dal Tadka ₹140.00`). Up/Down (`k`/`j` or Arrow keys) navigation.
   - **Current Order (Right Pane):** Interactive table (`Item`, `Price`, `Qty`, `Total`).
   - **Hotkeys:**
     - `Enter`: Add selected dish to cart or increment count.
     - `+` / `-`: Increase or decrease selected item quantity in cart.
     - `c` / `C`: Clear current active order.
     - `p` / `P`: Settle & print (persists to SQLite, exports itemized receipt to `receipt.txt`, and resets cart).
     - `q` / `Esc`: Cleanly exit the application.
   - **Financials:** Auto-computes Subtotal, composite 5% GST (2.5% CGST + 2.5% SGST), and rounded Grand Total.
   - **Receipt & Storage:** On checkout, record transaction into `dhabatui.db` and write a human-readable text receipt (`receipt.txt`).

3. **Engineering & Reliability Standards:**
   - Output every single file in full, including imports and implementation.
   - Safe terminal handling: custom panic hook and canonical terminal restoration (`LeaveAlternateScreen`, `disable_raw_mode`, cursor re-enabled).
   - Bundled SQLite persistence (`rusqlite` with `bundled` feature).
