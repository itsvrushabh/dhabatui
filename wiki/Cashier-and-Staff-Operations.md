# Cashier & Staff Operations

This operational manual is designed for cashiers, counter staff, and dhaba managers using **DhabaTUI** during daily business hours.

---

## ⌨️ Global Keybindings Cheatsheet

| Key | Action | Context |
| :---: | :--- | :--- |
| `[Tab]` / `[Shift+Tab]` | Switch between Tables | Main Tables view |
| `[Enter]` | Open Table / Confirm Action | Any view |
| `[A]` | Add Menu Item to active table | Table order screen |
| `[D]` | Delete / Decrement selected item | Table order screen |
| `[K]` | Generate Kitchen Order Ticket (KOT) | Table order screen |
| `[B]` | Finalize and Print Bill | Table order screen |
| `[H]` | View Bill History & Receipts | Main view |
| `[S]` | Day-End Shift & Sales Summary | Main view |
| `[M]` | Open Menu Management & Pricing | Main view |
| `[Esc]` | Cancel / Back / Return | Any modal / screen |
| `[Q]` | Exit DhabaTUI safely | Main view |

---

## 🍛 Typical Dhaba Order Workflow

### 1. Seating Guests & Punching Orders
1. Cashier uses `[Tab]` to highlight **Table 3** (shows `[Vacant]`).
2. Press `[Enter]` to open the table.
3. Press `[A]` to open quick menu entry:
   - Type item code (e.g., `M1` for Special Chai, `R2` for Dal Tadka) or use arrows.
   - Enter quantity (e.g. `2`) and press `[Enter]`.
4. Table subtotal automatically updates in paise.

### 2. Sending Order to Kitchen (KOT)
1. Press `[K]` to generate a **Kitchen Order Ticket (KOT)**.
2. The KOT records newly added items, timestamp, and table number, and resets the unprinted item count.
3. Kitchen staff receives the slip or views it via mobile dashboard.

### 3. Finalizing Bill & Payment
1. When guests request the bill, press `[B]`.
2. Select payment method:
   - `[1]` Cash
   - `[2]` UPI / QR Code (displays registered Dhaba UPI ID)
   - `[3]` Card
3. Press `[Enter]` to print receipt.
4. Bill is committed to the ACID store and Table 3 resets to `[Vacant]`.

---

## 📊 End-of-Day Shift Closure

At the end of the shift or day:
1. Press `[S]` to open the **Daily Settlement Report**.
2. View totals:
   - Total Gross Sales (₹)
   - Total GST Collected (CGST + SGST)
   - Cash in Drawer vs UPI receipts
   - Total Bills issued and table turnover rate
3. Press `[P]` to export or print the settlement summary.
