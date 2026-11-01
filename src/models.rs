use serde::{Deserialize, Serialize};

/// Master item defined in the menu catalog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MenuItem {
    pub code: String,
    pub name: String,
    pub category: String,
    /// Price stored in integer paise (1 Rupee = 100 Paise).
    pub price_paise: u32,
}

impl MenuItem {
    pub fn new(
        code: impl Into<String>,
        name: impl Into<String>,
        category: impl Into<String>,
        price_paise: u32,
    ) -> Self {
        Self {
            code: code.into(),
            name: name.into(),
            category: category.into(),
            price_paise,
        }
    }
}

/// An item line in the active billing cart.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CartItem {
    pub code: String,
    pub name: String,
    /// Unit price in paise. Can be altered on the go for this order.
    pub price_paise: u32,
    pub quantity: u32,
    /// Indicates whether this item was added as an ad-hoc custom/making charge.
    pub is_custom: bool,
}

impl CartItem {
    pub fn from_menu(item: &MenuItem, quantity: u32) -> Self {
        Self {
            code: item.code.clone(),
            name: item.name.clone(),
            price_paise: item.price_paise,
            quantity,
            is_custom: false,
        }
    }

    pub fn new_custom(
        code: impl Into<String>,
        name: impl Into<String>,
        price_paise: u32,
        quantity: u32,
    ) -> Self {
        Self {
            code: code.into(),
            name: name.into(),
            price_paise,
            quantity,
            is_custom: true,
        }
    }

    /// Computes the total for this line item in paise.
    pub fn line_total_paise(&self) -> u32 {
        self.price_paise.saturating_mul(self.quantity)
    }
}

/// Aggregated financial summary for an order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct BillSummary {
    pub subtotal_paise: u32,
    /// Central GST (2.5%).
    pub cgst_paise: u32,
    /// State GST (2.5%).
    pub sgst_paise: u32,
    /// Grand total payable in paise.
    pub total_paise: u32,
    /// Total count of physical items (sum of quantities).
    pub total_items: u32,
    /// Total number of distinct cart lines.
    pub total_lines: usize,
}

impl BillSummary {
    /// Computes financial breakdown from a slice of cart items.
    /// CGST is 2.5% and SGST is 2.5%, rounded to the nearest paise.
    pub fn calculate(items: &[CartItem]) -> Self {
        let mut subtotal: u64 = 0;
        let mut total_items: u64 = 0;

        for item in items {
            subtotal = subtotal.saturating_add(item.line_total_paise() as u64);
            total_items = total_items.saturating_add(item.quantity as u64);
        }

        // 2.5% tax: (subtotal * 25 + 500) / 1000 for standard half-up rounding
        let cgst = if subtotal > 0 {
            (subtotal.saturating_mul(25) + 500) / 1000
        } else {
            0
        };

        let sgst = if subtotal > 0 {
            (subtotal.saturating_mul(25) + 500) / 1000
        } else {
            0
        };

        let total = subtotal.saturating_add(cgst).saturating_add(sgst);

        Self {
            subtotal_paise: subtotal as u32,
            cgst_paise: cgst as u32,
            sgst_paise: sgst as u32,
            total_paise: total as u32,
            total_items: total_items as u32,
            total_lines: items.len(),
        }
    }
}

/// Final historical invoice record stored in redb.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BillRecord {
    pub id: u64,
    pub timestamp: String,
    pub items: Vec<CartItem>,
    pub summary: BillSummary,
}

/// Formats integer paise into a standard Rupee string (`₹XX.YY`).
pub fn format_paise(paise: u32) -> String {
    let rupees = paise / 100;
    let fraction = paise % 100;
    format!("₹{}.{:02}", rupees, fraction)
}

/// Parses an input string representing Rupees (e.g. "140", "140.50", "₹180.00")
/// into integer paise.
pub fn parse_rupees_to_paise(input: &str) -> Result<u32, String> {
    let cleaned = input
        .trim()
        .trim_start_matches('₹')
        .trim_start_matches("Rs")
        .trim_start_matches("rs")
        .trim_start_matches("RS")
        .trim();

    if cleaned.is_empty() {
        return Err("Price cannot be empty".to_string());
    }

    if let Some((rupee_str, paise_str)) = cleaned.split_once('.') {
        let rupees: u32 = rupee_str
            .parse()
            .map_err(|_| format!("Invalid rupee amount: '{rupee_str}'"))?;

        let paise_fraction: u32 = match paise_str.len() {
            0 => 0,
            1 => {
                let digit: u32 = paise_str
                    .parse()
                    .map_err(|_| format!("Invalid paise decimal: '{paise_str}'"))?;
                digit * 10
            }
            2 => paise_str
                .parse()
                .map_err(|_| format!("Invalid paise decimal: '{paise_str}'"))?,
            _ => {
                // Truncate/round to 2 decimals
                let two_digits = &paise_str[..2];
                two_digits
                    .parse()
                    .map_err(|_| format!("Invalid paise decimal: '{paise_str}'"))?
            }
        };

        rupees
            .checked_mul(100)
            .and_then(|r| r.checked_add(paise_fraction))
            .ok_or_else(|| "Price value overflowed".to_string())
    } else {
        let rupees: u32 = cleaned
            .parse()
            .map_err(|_| format!("Invalid rupee number: '{cleaned}'"))?;
        rupees
            .checked_mul(100)
            .ok_or_else(|| "Price value overflowed".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_paise() {
        assert_eq!(format_paise(0), "₹0.00");
        assert_eq!(format_paise(50), "₹0.50");
        assert_eq!(format_paise(13000), "₹130.00");
        assert_eq!(format_paise(18055), "₹180.55");
    }

    #[test]
    fn test_parse_rupees_to_paise() {
        assert_eq!(parse_rupees_to_paise("130").unwrap(), 13000);
        assert_eq!(parse_rupees_to_paise("130.00").unwrap(), 13000);
        assert_eq!(parse_rupees_to_paise("130.5").unwrap(), 13050);
        assert_eq!(parse_rupees_to_paise("₹210.75").unwrap(), 21075);
        assert_eq!(parse_rupees_to_paise("Rs 50").unwrap(), 5000);
        assert!(parse_rupees_to_paise("-10").is_err());
        assert!(parse_rupees_to_paise("abc").is_err());
    }

    #[test]
    fn test_bill_calculation_and_gst() {
        let items = vec![
            CartItem {
                code: "101".to_string(),
                name: "Dal Tadka".to_string(),
                price_paise: 13000,
                quantity: 2,
                is_custom: false,
            },
            CartItem {
                code: "202".to_string(),
                name: "Butter Naan".to_string(),
                price_paise: 4500,
                quantity: 4,
                is_custom: false,
            },
        ];

        let summary = BillSummary::calculate(&items);
        // Subtotal = 2 * 13000 + 4 * 4500 = 26000 + 18000 = 44000 (₹440.00)
        assert_eq!(summary.subtotal_paise, 44000);
        // CGST (2.5%) = 44000 * 25 / 1000 = 1100 (₹11.00)
        assert_eq!(summary.cgst_paise, 1100);
        // SGST (2.5%) = 1100 (₹11.00)
        assert_eq!(summary.sgst_paise, 1100);
        // Grand Total = 44000 + 1100 + 1100 = 46200 (₹462.00)
        assert_eq!(summary.total_paise, 46200);
        assert_eq!(summary.total_items, 6);
        assert_eq!(summary.total_lines, 2);
    }
}
