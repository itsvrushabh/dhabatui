# 🍲 DhabaTUI

[![CI](https://github.com/itsvrushabh/dhabatui/actions/workflows/ci.yml/badge.svg)](https://github.com/itsvrushabh/dhabatui/actions/workflows/ci.yml)
[![Rust](https://img.shields.io/badge/Rust-2021_Edition-orange?logo=rust)](https://www.rust-lang.org/)
[![TUI](https://img.shields.io/badge/TUI-Ratatui_0.29-blue)](https://github.com/ratatui-org/ratatui)
[![Database](https://img.shields.io/badge/Database-redb_2.x_ACID-teal)](https://github.com/cberner/redb)
[![License](https://img.shields.io/badge/License-MIT-green)](LICENSE)

A lightning-fast, keyboard-driven Terminal User Interface (TUI) Point-of-Sale (POS) and billing application built specifically for Indian dhabas, roadside eateries, and small-scale restaurants.

---

## 🖥️ Live Terminal Interface

![DhabaTUI Screenshot](assets/screenshot.svg)

```text
┌─ [1] Menu Catalog ▶ (Enter: Add | Digits: Quick Code) ──┬─ [2] Current Order ▶ (+/- Qty | e: Edit Price | d: Del) ┐
│ Code  Item Name             Category       Price        │ Code  Item Name                 Qty  Unit Price     Total │
│ ▶ 101 Dal Tadka             Mains        ₹130.00        │ [CUST]* 1/2kg Mutton Masala (M)   1     ₹180.00   ₹180.00 │
│   102 Dal Makhani           Mains        ₹160.00        │ [CUST]* 1/2kg Mutton Fry (Making) 1     ₹160.00   ₹160.00 │
│   103 Paneer Butter Masala  Mains        ₹210.00        │ [201] Tandoori Roti               6      ₹15.00    ₹90.00 │
│   201 Tandoori Roti         Breads        ₹15.00        │ [302] Jeera Rice                  1     ₹120.00   ₹120.00 │
│   202 Butter Naan           Breads        ₹45.00        │ [502] Chaas / Buttermilk          2      ₹30.00    ₹60.00 │
│   401 Mutton Making (1/2kg) Making Chg   ₹180.00        ├─────────────────────────────────────────────────────────┤
│   402 Mutton Making (1 kg)  Making Chg   ₹320.00        │ Total Items: 11 (5 lines)          Subtotal:    ₹610.00 │
│   501 Sweet Lassi           Beverages     ₹60.00        │                                    CGST (2.5%):  ₹15.25 │
│   502 Chaas / Buttermilk    Beverages     ₹30.00        │                                    SGST (2.5%):  ₹15.25 │
│                                                         │                                    GRAND TOTAL: ₹640.50 │
├─────────────────────────────────────────────────────────┴─────────────────────────────────────────────────────────┤
│ ✓ Bill #101 settled (₹640.50)! Saved to receipt.txt and dhabatui.redb                                             │
├───────────────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ Tab:Pane | Enter:Add | +/-:Qty | e:Edit Price | m:Making Chg | d:Del | c:Clear | p:Settle & Print | q:Quit        │
└───────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘
```

---

## 📖 How to Use DhabaTUI (Cashier's Guide)

DhabaTUI is engineered so the cashier never needs to take their hands off the keyboard.

### 1. Launching the Counter
Start DhabaTUI from any terminal by typing:
```bash
dhabatui
# or simply
dti
```
The application opens in an alternate screen without altering your shell history.

---

### 2. Adding Standard Dishes
- **Browse & Add**: Use `j` / `k` (or `↑` / `↓`) to scroll through the Menu Catalog on the left. Press `Enter` to add the highlighted item to the active bill.
- **Auto-Increment**: Adding the same item multiple times increments its quantity automatically in the cart.

---

### 3. Rapid Quick-Code Entry (Rush Hours)
Dhaba cashiers often memorize item numbers:
- Simply type the dish code numbers directly on your keyboard (e.g. type `1`, `0`, `1`).
- A red `[Code: 101_]` indicator appears at the top.
- Press `Enter` to immediately add it to the order.
- Press `Esc` or `Backspace` to cancel or correct digits.

---

### 4. Customer-Supplied Raw Meat / Making Charges (*Banwai*)
A signature feature of highway and rural dhabas: customers bring their own raw meat, chicken, or fish (e.g. *1 kg mutton*) and ask the cook to prepare it into separate dishes (*half kg masala, half kg fry*):

1. Press **`m`** anywhere to open the **Making Charge Modal**.
2. **Step 1**: Type the custom description (e.g., `1/2kg Mutton Masala (Making)`) and press `Enter`.
3. **Step 2**: Enter the negotiated cooking charge (e.g., `180.00`) and press `Enter`.
4. The dish is immediately added to the bill with a special `*` custom marker!

*(Tip: You can also quick-code standard making rates like `401` for Mutton 1/2kg, `402` for Mutton 1kg, etc.)*

---

### 5. Altering Prices on the Go
To give a verbal bulk discount or change a price for the current order:
1. Press **`Tab`** or **`l`** (Right Arrow) to switch focus to the **Current Order** pane.
2. Use `j` / `k` to highlight the item you want to adjust.
3. Press **`e`** to open the **Alter Unit Price** modal.
4. Type the new unit price (e.g. `160.00`) and press `Enter`.
5. The line total, subtotal, and GST recalculate instantly for this order without altering the permanent menu catalog.

---

### 6. Adjusting Quantities & Line Items
With the **Current Order** pane focused:
- **`+`** or **`=`**: Increase quantity by 1.
- **`-`**: Decrease quantity by 1 (removes line if it reaches 0).
- **`d`**: Delete the highlighted line item completely.
- **`c`**: Clear the entire active order.

---

### 7. Settle & Print Receipt
When the customer is ready to pay:
- Press **`p`** anywhere.
- The order is committed to `dhabatui.redb` under a sequential invoice number (e.g. `#101`).
- The itemized bill is exported to [`receipt.txt`](receipt.txt) for 80mm thermal ESC/POS printers and archived to `receipts/bill_<id>.txt`.
- The cart resets to blank and a green confirmation banner confirms settlement.

---

## ⌨️ Keyboard Shortcuts Reference

| Shortcut | Context | Description |
| :--- | :--- | :--- |
| **`Tab`** | Global | Toggle active focus between **Menu Catalog** and **Current Order** |
| **`h`** / **`Left`** | Global | Focus **Menu Catalog** |
| **`l`** / **`Right`** | Global | Focus **Current Order** |
| **`j`** / **`Down`** | Active Pane | Move selection down |
| **`k`** / **`Up`** | Active Pane | Move selection up |
| **`0`–`9`** | Menu Pane | Direct numeric quick code entry (e.g. `101` + `Enter`) |
| **`Enter`** | Menu Pane | Add selected dish or increment count in cart |
| **`+`** / **`=`** | Cart Pane | Increase quantity of selected item |
| **`-`** | Cart Pane | Decrease quantity of selected item |
| **`e`** / **`E`** | Cart Pane | **Alter price on the go**: modal prompt to override unit price |
| **`m`** / **`M`** | Global | **Add Custom / Making Charge**: modal for dish description & price |
| **`d`** / **`D`** | Cart Pane | Delete selected item line from cart |
| **`c`** / **`C`** | Global | Clear entire cart |
| **`p`** / **`P`** | Global | **Settle & Print**: commit to `redb`, write `receipt.txt`, and clear cart |
| **`q`** / **`Esc`** | Global | Cleanly exit DhabaTUI (or dismiss open popup modal) |

---

## 🧾 Sample Printed Receipt (`receipt.txt`)

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

## 📚 Technical Documentation

All internal code architecture, financial math, and developer documentation have been moved to the [`docs/`](docs/) directory:

- 🏗️ **[System Architecture](docs/ARCHITECTURE.md)**: `redb` ACID storage engine, MVCC transactions, and non-blocking file logging (`dhabatui.log`).
- 💰 **[Financials & GST Math](docs/FINANCIALS.md)**: Integer paise representation and exact half-up rounding for 2.5% CGST + 2.5% SGST.
- 🛠️ **[Developer & Testing Guide](docs/DEVELOPMENT.md)**: Building, running unit tests, CI/CD pipeline, and contributing.

---

## 📄 License

This project is licensed under the MIT License.
