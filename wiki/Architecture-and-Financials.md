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
Every currency amount is modeled as an unsigned 64-bit integer (`u64`) representing **paise** ($1\text{ Rupee} = 100\text{ paise}$):

```rust
// 100 paise = ₹1.00
pub type Paise = u64;

pub fn format_paise(paise: Paise) -> String {
    let rupees = paise / 100;
    let remainder = paise % 100;
    format!("₹{}.{:02}", rupees, remainder)
}
```

### GST Calculation
GST is calculated using integer multiplication and integer division with standard half-up rounding:
$$\text{GST Paise} = \frac{\text{Taxable Paise} \times \text{Rate Percent} + 50}{100}$$

```rust
pub fn calculate_gst(taxable_paise: u64, gst_percent: u64) -> u64 {
    (taxable_paise * gst_percent + 50) / 100
}
```

---

## 🗄️ Persistence & Storage Architecture

DhabaTUI utilizes **Redb** (`redb = "2"`), an embedded, pure-Rust, transactional key-value store:

- **ACID Compliant**: Guarantees atomic transaction boundaries for all bill saves and status transitions.
- **Single File (`dhabatui.redb`)**: Zero external database servers (no PostgreSQL, MySQL, or SQLite required).
- **Crash Resilient**: If power cuts or accidental terminal closures occur during rush hour, transactions are either fully committed or safely rolled back.

### Tables in Redb:
1. `tables`: Current table occupancy, active items, and subtotal.
2. `bills`: Historical finalized bills with timestamp, payment method, and itemized breakdown.
3. `menu`: Active menu catalog, item codes, and prices.
4. `config`: Dhaba profile (name, phone, address, GSTIN, UPI ID).

---

## 🌐 Local LAN HTTP Dashboard

DhabaTUI embeds a lightweight HTTP server via `tiny_http`:
- Binds to `0.0.0.0:8080`.
- Automatically detects and displays local LAN IP (e.g., `http://192.168.1.45:8080`).
- Serves:
  - Responsive HTML/CSS/JS dashboard for mobile phones and tablets.
  - JSON API endpoints:
    - `GET /api/status`: Active tables, totals, and occupancy.
    - `GET /api/menu`: Menu items with pricing.
    - `GET /api/reports`: Daily settlement and shift summaries.
- Zero external network access needed — functions seamlessly over local Wi-Fi or mobile hotspots without internet.
