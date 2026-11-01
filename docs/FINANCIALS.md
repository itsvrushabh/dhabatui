# DhabaTUI — Financial Calculation & GST Specification

This document details the monetary representation, Indian GST compliance, and thermal receipt generation logic used in `dhabatui`.

---

## 1. Monetary Representation: Integer Paise

Floating-point numbers (`f64` / `f32`) suffer from IEEE-754 precision issues (e.g. `0.1 + 0.2 = 0.30000000000000004`). In point-of-sale systems, even a 1-paise discrepancy causes accounting mismatches.

DhabaTUI stores all prices, totals, and taxes internally as **integer paise**:
$$1\text{ Indian Rupee (₹)} = 100\text{ Paise}$$

### Examples
- ₹130.00 = `13000` paise
- ₹15.00 = `1500` paise
- ₹0.50 = `50` paise

### Conversion Helpers
```rust
// Formatting paise into display string
pub fn format_paise(paise: u32) -> String {
    let rupees = paise / 100;
    let fraction = paise % 100;
    format!("₹{}.{:02}", rupees, fraction)
}

// Parsing user input ("140", "140.50", "₹180") into integer paise
pub fn parse_rupees_to_paise(input: &str) -> Result<u32, String> { ... }
```

---

## 2. Indian GST Compliance (5% Composite GST)

Standard Indian restaurant tax structures for non-AC and casual dining apply a composite **5% GST**, split equally into:
- **Central GST (CGST)**: 2.5%
- **State GST (SGST)**: 2.5%

### Exact Mathematical Formulas
To ensure fair rounding to the nearest integer paise without truncation bias, DhabaTUI applies half-up rounding:

$$\text{Subtotal} = \sum_{i=1}^{n} (\text{Item Price}_i \times \text{Quantity}_i)$$

$$\text{CGST} = \left\lfloor \frac{\text{Subtotal} \times 25 + 500}{1000} \right\rfloor$$

$$\text{SGST} = \left\lfloor \frac{\text{Subtotal} \times 25 + 500}{1000} \right\rfloor$$

$$\text{Grand Total} = \text{Subtotal} + \text{CGST} + \text{SGST}$$

### Worked Example
For an order of:
- 1x 1/2kg Mutton Masala (Making): ₹180.00 (`18000` paise)
- 1x 1/2kg Mutton Fry (Making): ₹160.00 (`16000` paise)
- 6x Tandoori Roti: ₹15.00 each (`9000` paise)
- 1x Jeera Rice: ₹120.00 (`12000` paise)
- 2x Chaas / Buttermilk: ₹30.00 each (`6000` paise)

$$\text{Subtotal} = 18000 + 16000 + 9000 + 12000 + 6000 = 61000\text{ paise } (₹610.00)$$

$$\text{CGST} = \left\lfloor \frac{61000 \times 25 + 500}{1000} \right\rfloor = \left\lfloor \frac{1525000 + 500}{1000} \right\rfloor = 1525\text{ paise } (₹15.25)$$

$$\text{SGST} = \left\lfloor \frac{61000 \times 25 + 500}{1000} \right\rfloor = 1525\text{ paise } (₹15.25)$$

$$\text{Grand Total} = 61000 + 1525 + 1525 = 64050\text{ paise } (₹640.50)$$

---

## 3. Thermal Receipt Format

When an order is settled (`p`), DhabaTUI writes a 60-column plain-text receipt formatted for direct printing to 3-inch (80mm) thermal ESC/POS receipt printers:

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

- **Primary Receipt**: Saved to [`receipt.txt`](file:///home/cachyos/Work/dhabatui/receipt.txt).
- **Audit Archive**: Saved to [`receipts/bill_<id>.txt`](file:///home/cachyos/Work/dhabatui/receipts/).
