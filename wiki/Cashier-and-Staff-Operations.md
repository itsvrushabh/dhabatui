# Cashier & Staff Operations

This operational manual is designed for cashiers, counter staff, and dhaba managers using **DhabaTUI** during daily business hours.

---

## ⌨️ Global Keybindings Cheatsheet

| Key | Action | Context |
| :---: | :--- | :--- |
| `[t]` / `[T]` | Cycle Dining Tables (`T1`–`T8`, Parcel) | Table bar at top |
| `[Shift+Tab]` | Cycle Dining Tables Backward | Table bar at top |
| `[s]` | Cycle Table Status: `Vacant` ➔ `Ordering` ➔ `Served` ➔ `Billed` ➔ `Ordering` (resets on settle `[p]` or clear `[c]`) | Table bar at top |
| `[Tab]` | Switch Focus (Menu Catalog ⟷ Order Cart) | Main panes |
| `[Enter]` | Add highlighted item to order / Confirm modal | Menu Catalog |
| `[0]`–`[9]` + `[Enter]` | Rapid 3-digit dish code entry (e.g. `101`) | Fast counter ordering |
| `[+]` / `[-]` | Increase or decrease item quantity | Order Cart |
| `[m]` | Add custom meat cooking fee (*Banwai*) | Modal dialog |
| `[e]` | Alter unit price / discount on-the-fly | Order Cart |
| `[p]` | Settle Bill, calculate GST & save `receipt.txt` | Active table |
| `[c]` | Clear all items from active table | Active table |
| `[q]` / `[Esc]` | Close modal dialog / Exit DhabaTUI | Any screen |

---

## 🍛 Typical Dhaba Order Workflow

### 1. Seating Guests & Punching Orders
1. Cashier presses `[t]` to cycle to the table (e.g., **T3**).
2. Status will automatically update from `[Vacant]` to `[Ordering]` upon adding the first item, or press `[s]` to set manually.
3. Add items:
   - **Quick code**: Type dish code (e.g., `101` for Dal Tadka) and press `[Enter]`.
   - **Menu browse**: Use `↑` / `↓` in the Menu Catalog pane and press `[Enter]`.
4. Table subtotal automatically updates in paise.

### 2. Customer Meat Cooking (*Banwai*)
When guests bring their own raw meat/chicken:
1. Press `[m]` to open the Making Charge modal.
2. Enter custom dish description (e.g., `1/2kg Mutton Masala (Making)`) and press `[Enter]`.
3. Enter negotiated cooking charge in rupees (e.g., `180.00`) and press `[Enter]`.
4. The dish is added to the cart with a special `*` custom indicator.

### 3. Adjusting Quantities or On-the-Fly Prices
1. Press `[Tab]` to focus the Order Cart pane.
2. Use `↑` / `↓` to select an item.
3. Press `[+]` or `[-]` to adjust quantity.
4. Press `[e]` to alter unit price on-the-fly (e.g., regular customer discount).

### 4. Finalizing Bill & Receipt
1. When guests request the bill, press `[p]`.
2. DhabaTUI settles the bill, applies GST half-up rounding in exact paise, resets the table to `[Vacant]`, and outputs a formatted bill to `receipt.txt`.
