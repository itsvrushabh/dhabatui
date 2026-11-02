# Architecture & Financials

This document details the architectural principles, persistence guarantees, and paise-precision financial arithmetic that underpin **DhabaTUI**.

---

## 💰 The Zero-Float Financial Invariant

> [!IMPORTANT]
> **Floating-point types (`f32`, `f64`) are strictly forbidden** for currency values throughout the entire DhabaTUI codebase.

### Why Floating Point is Forbidden
In standard binary floating point (IEEE 754), fractional decimal values cannot be represented exactly:
$$0.1_{10} + 0.2_{10} = 0.30000000000000004_{10}$$
In a busy dhaba serving hundreds of customers, accumulated floating-point rounding errors cause:
1. Daily register reconciliation mismatches.
2. Inconsistent split bills across table seats.
3. Rounding disputes with customers and tax authorities.

### The Integer Paise Representation
Every currency amount is modeled as an integer representing **paise** ($1\text{ Rupee} = 100\text{ paise}$):

```rust
// 100 paise = ₹1.00
pub fn format_paise(paise: u32) -> String {
    let rupees = paise / 100;
    let remainder = paise % 100;
    format!("₹{}.{:02}", rupees, remainder)
}
```

### GST Calculation
CGST (2.5%) and SGST (2.5%) are calculated in `BillSummary::calculate()` using integer multiplication and integer division with standard half-up rounding:
$$\text{Tax Paise} = \frac{\text{Subtotal Paise} \times 25 + 500}{1000}$$

```rust
// 2.5% tax: (subtotal * 25 + 500) / 1000 for standard half-up rounding
let cgst = if subtotal > 0 {
    (subtotal.saturating_mul(25) + 500) / 1000
} else {
    0
};
let sgst = cgst;
let total = subtotal.saturating_add(cgst).saturating_add(sgst);
```

---

## 🗄️ Persistence & Storage Architecture

DhabaTUI utilizes **Redb** (`redb = "2"`), an embedded, pure-Rust, transactional key-value store:

- **ACID Compliant**: Guarantees atomic transaction boundaries for all bill saves and status transitions.
- **Single File (`dhabatui.redb`)**: Zero external database servers (no PostgreSQL, MySQL, or SQLite required).
- **Crash Resilient**: If power cuts or accidental terminal closures occur during rush hour, transactions are either fully committed or safely rolled back.

### Tables in Redb:
1. `menu_items`: Default catalog items, 3-digit dish codes, category, and unit price in paise.
2. `bills`: Historical finalized bills (`u64` bill ID sequence to serialized `BillRecord` with exact paise calculations).
3. `metadata`: Sequence counter for atomic bill ID generation and schema metadata.

---

## 🌐 Local LAN HTTP Dashboard

DhabaTUI embeds a lightweight HTTP server via `tiny_http`:
- Binds to `0.0.0.0:8080` (or falls back to subsequent available ports).
- Automatically detects and displays local LAN IP (e.g., `http://192.168.1.45:8080`).
- Serves:
  - Responsive HTML/CSS/JS dashboard for mobile phones and tablets to view dining hall table orders in real time.
  - JSON API endpoint:
    - `GET /api/status`: Active tables, occupancy, items, totals in paise, and recent bills.
- Zero external network access needed — functions seamlessly over local Wi-Fi or mobile hotspots without internet.
