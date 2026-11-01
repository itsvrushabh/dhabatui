# 🍲 DhabaTUI

A fast, keyboard-first Terminal User Interface (TUI) Point-of-Sale (POS) and billing application engineered specifically for Indian dhabas, highway eateries, and small-scale restaurants.

Built in 100% pure, safe Rust using **[Ratatui](https://github.com/ratatui-org/ratatui)**, **[Crossterm](https://github.com/crossterm-rs/crossterm)**, and an embedded **[redb](https://github.com/cberner/redb)** ACID key-value database engine.

---

## ✨ Features

- **⚡ Blazing Fast Keyboard Operation**: Run the entire billing counter without touching a mouse.
- **🛡️ 100% Pure Safe Rust & Zero C Dependencies**: Powered by `redb 2.x` copy-on-write B-tree storage for crash-resilient ACID durability on sudden power cuts.
- **🥩 Real-World Dhaba Workflow ("Making Charges" / *Banwai*)**:
  - Customers frequently bring their own meat/poultry/fish (e.g., *1 kg raw mutton*) and ask the dhaba to cook it into different dishes (*1/2 kg Mutton Masala* + *1/2 kg Mutton Fry*).
  - Press `m` to instantly add custom cooking/making charges on the fly with arbitrary descriptions and prices.
- **✏️ On-the-Go Price Alteration**:
  - Press `e` on any selected cart item to alter its unit price for the active bill without modifying the permanent master menu catalog.
- **🔢 Direct Quick Code Entry**:
  - Cashiers can type dish numbers directly (e.g. `101`, `202`, `401`) and press `Enter` to instantly add or increment items in the order.
- **💰 100% Exact Financial Math (Integer Paise)**:
  - All financial calculations are computed in integer paise (`₹1.00 = 100 paise`), eliminating floating-point rounding errors on composite 5% GST (2.5% CGST + 2.5% SGST).
- **📝 Instant Thermal Receipt Generation**:
  - On checkout (`p`), the transaction is atomically committed to `dhabatui.redb`, exported to [`receipt.txt`](receipt.txt) for ESC/POS printing, and archived into `receipts/bill_<id>.txt`.
- **🪵 Safe Asynchronous File Logging**:
  - All audit trails, database commits, and runtime diagnostics are logged to [`dhabatui.log`](dhabatui.log) via non-blocking `tracing-appender`, completely isolating terminal alternate screens from log corruption.
- **🚨 Fail-Safe Terminal Restoration**:
  - Custom panic hook guarantees that terminal raw mode is disabled and the alternate screen is exited even if an unexpected panic occurs.

---

## 🚀 Quick Start

### Build & Run

```bash
# Clone and enter directory
cd /home/cachyos/Work/dhabatui

# Run directly with Cargo
cargo run --release

# Run unit & integration tests
cargo test
```

---

## ⌨️ Keyboard Shortcuts

| Shortcut | Context | Description |
| :--- | :--- | :--- |
| **`Tab`** | Global | Toggle active pane focus between **Menu Catalog** and **Current Order** |
| **`h`** / **`Left`** | Global | Jump focus to **Menu Catalog** |
| **`l`** / **`Right`** | Global | Jump focus to **Current Order** |
| **`j`** / **`Down`** | Active Pane | Move selection down |
| **`k`** / **`Up`** | Active Pane | Move selection up |
| **`0`–`9`** | Menu Pane | Direct quick code entry (e.g. `101` + `Enter`) |
| **`Enter`** | Menu Pane | Add selected dish or increment count in cart |
| **`+`** / **`=`** | Cart Pane | Increase quantity of selected cart line |
| **`-`** | Cart Pane | Decrease quantity (removes line if quantity reaches 0) |
| **`e`** / **`E`** | Cart Pane | **Alter price on the go**: opens popup modal to override unit price |
| **`m`** / **`M`** | Global | **Add Custom / Making Charge**: step-by-step modal for item name and price |
| **`d`** / **`D`** | Cart Pane | Delete selected line item from order |
| **`c`** / **`C`** | Global | Clear entire cart |
| **`p`** / **`P`** | Global | **Settle & Print**: commit order to `redb`, write `receipt.txt`, and clear cart |
| **`q`** / **`Esc`** | Global | Exit application (or dismiss modal if open) |

---

## 📜 Sample Generated Receipt (`receipt.txt`)

```text
============================================================
                      DHABA TUI BILL
============================================================
Bill No: #101                      Date: 2026-11-01 12:25:30
------------------------------------------------------------
Item                            Qty    Price        Total
------------------------------------------------------------
* 1/2kg Mutton Masala (Making)    1  ₹180.00      ₹180.00
* 1/2kg Mutton Fry (Making)       1  ₹160.00      ₹160.00
Tandoori Roti                     6   ₹15.00       ₹90.00
Jeera Rice                        1  ₹120.00      ₹120.00
Chaas / Buttermilk                2   ₹30.00       ₹60.00
------------------------------------------------------------
Total Items: 11 (5 lines)
Subtotal:                                         ₹610.00
CGST (2.5%):                                       ₹15.25
SGST (2.5%):                                       ₹15.25
------------------------------------------------------------
GRAND TOTAL:                                      ₹640.50
============================================================
                 Thank You! Visit Again!
============================================================
```

---

## 🏗️ Project Architecture

```
dhabatui/
├── Cargo.toml          # Rust dependencies & metadata
├── src/
│   ├── main.rs         # Terminal lifecycle, panic hook, and 250ms event loop
│   ├── app.rs          # State machine, cart actions, on-the-go pricing logic
│   ├── db.rs           # redb storage engine, default menu seeds, atomic billing
│   ├── models.rs       # Domain entities, paise math, and 2.5% CGST/SGST tax
│   ├── logger.rs       # Non-blocking file logging to dhabatui.log
│   └── ui.rs           # Pure Ratatui layout, tables, lists, and modal popups
├── receipts/           # Archived bills (bill_101.txt, bill_102.txt...)
├── receipt.txt         # Latest generated thermal bill
└── dhabatui.redb       # Embedded ACID redb database file
```

---

## 🧪 Testing

Run the full suite of unit and workflow tests:

```bash
cargo test
```

### Verified Test Cases
1. `models::tests::test_bill_calculation_and_gst`: Verifies accurate 2.5% CGST and SGST splits and rounding on integer paise.
2. `models::tests::test_format_paise` & `test_parse_rupees_to_paise`: Tests currency formatting and user string parsing (`₹`, `Rs`, decimals).
3. `db::tests::test_db_initialization_and_seed`: Tests initial catalog population into `redb`.
4. `db::tests::test_save_bill_transaction`: Asserts atomic commit, sequential bill IDs, and receipt file generation.
5. `app::tests::test_dhaba_mutton_making_workflow`: Simulates the complete real-world customer meat preparation workflow, on-the-fly price alteration, item additions, and settlement.
