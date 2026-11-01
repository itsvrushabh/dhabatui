use crate::db::{Database, DbError};
use crate::models::{
    format_paise, parse_rupees_to_paise, BillRecord, BillSummary, CartItem, MenuItem,
};
use ratatui::widgets::{ListState, TableState};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivePane {
    Menu,
    Cart,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    QuickCode,
    EditingPrice,
    AddingCustomName,
    AddingCustomPrice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastType {
    Success,
    Info,
    Error,
}

pub struct App {
    pub db: Database,
    pub menu: Vec<MenuItem>,
    pub menu_state: ListState,
    pub cart: Vec<CartItem>,
    pub cart_state: TableState,
    pub active_pane: ActivePane,
    pub input_mode: InputMode,
    pub quick_code_buffer: String,
    pub input_buffer: String,
    pub pending_custom_name: String,
    pub toast: Option<(String, Instant, ToastType)>,
    pub should_quit: bool,
}

impl App {
    pub fn new(db: Database) -> Result<Self, DbError> {
        let menu = db.get_menu()?;
        let mut menu_state = ListState::default();
        if !menu.is_empty() {
            menu_state.select(Some(0));
        }

        let mut cart_state = TableState::default();
        cart_state.select(None);

        Ok(Self {
            db,
            menu,
            menu_state,
            cart: Vec::new(),
            cart_state,
            active_pane: ActivePane::Menu,
            input_mode: InputMode::Normal,
            quick_code_buffer: String::new(),
            input_buffer: String::new(),
            pending_custom_name: String::new(),
            toast: Some((
                "Welcome to DhabaTUI! Press '?' or see keys below.".to_string(),
                Instant::now(),
                ToastType::Info,
            )),
            should_quit: false,
        })
    }

    /// Periodic tick to clear expired toasts.
    pub fn tick(&mut self) {
        if let Some((_, timestamp, _)) = &self.toast {
            if timestamp.elapsed() > Duration::from_secs(4) {
                self.toast = None;
            }
        }
    }

    pub fn set_toast(&mut self, message: impl Into<String>, toast_type: ToastType) {
        self.toast = Some((message.into(), Instant::now(), toast_type));
    }

    // --- Navigation ---

    pub fn next_item(&mut self) {
        match self.active_pane {
            ActivePane::Menu => {
                if self.menu.is_empty() {
                    return;
                }
                let i = match self.menu_state.selected() {
                    Some(i) => {
                        if i >= self.menu.len() - 1 {
                            0
                        } else {
                            i + 1
                        }
                    }
                    None => 0,
                };
                self.menu_state.select(Some(i));
            }
            ActivePane::Cart => {
                if self.cart.is_empty() {
                    return;
                }
                let i = match self.cart_state.selected() {
                    Some(i) => {
                        if i >= self.cart.len() - 1 {
                            0
                        } else {
                            i + 1
                        }
                    }
                    None => 0,
                };
                self.cart_state.select(Some(i));
            }
        }
    }

    pub fn previous_item(&mut self) {
        match self.active_pane {
            ActivePane::Menu => {
                if self.menu.is_empty() {
                    return;
                }
                let i = match self.menu_state.selected() {
                    Some(i) => {
                        if i == 0 {
                            self.menu.len() - 1
                        } else {
                            i - 1
                        }
                    }
                    None => 0,
                };
                self.menu_state.select(Some(i));
            }
            ActivePane::Cart => {
                if self.cart.is_empty() {
                    return;
                }
                let i = match self.cart_state.selected() {
                    Some(i) => {
                        if i == 0 {
                            self.cart.len() - 1
                        } else {
                            i - 1
                        }
                    }
                    None => 0,
                };
                self.cart_state.select(Some(i));
            }
        }
    }

    pub fn toggle_pane(&mut self) {
        self.active_pane = match self.active_pane {
            ActivePane::Menu => {
                if !self.cart.is_empty() && self.cart_state.selected().is_none() {
                    self.cart_state.select(Some(0));
                }
                ActivePane::Cart
            }
            ActivePane::Cart => ActivePane::Menu,
        };
    }

    // --- Cart Actions ---

    pub fn add_selected_menu_item(&mut self) {
        if let Some(i) = self.menu_state.selected() {
            if let Some(menu_item) = self.menu.get(i).cloned() {
                self.add_or_increment_menu_item(&menu_item);
            }
        }
    }

    pub fn add_item_by_code(&mut self, code: &str) -> bool {
        let trimmed = code.trim();
        if let Some(item) = self.menu.iter().find(|m| m.code.eq_ignore_ascii_case(trimmed)).cloned() {
            self.add_or_increment_menu_item(&item);
            true
        } else {
            self.set_toast(
                format!("Item with code '{trimmed}' not found"),
                ToastType::Error,
            );
            false
        }
    }

    fn add_or_increment_menu_item(&mut self, item: &MenuItem) {
        if let Some(pos) = self
            .cart
            .iter()
            .position(|c| !c.is_custom && c.code == item.code && c.price_paise == item.price_paise)
        {
            self.cart[pos].quantity = self.cart[pos].quantity.saturating_add(1);
            self.cart_state.select(Some(pos));
            self.set_toast(
                format!(
                    "Incremented '{}' (Qty: {})",
                    self.cart[pos].name, self.cart[pos].quantity
                ),
                ToastType::Success,
            );
        } else {
            let cart_item = CartItem::from_menu(item, 1);
            self.cart.push(cart_item);
            let new_pos = self.cart.len() - 1;
            self.cart_state.select(Some(new_pos));
            self.set_toast(
                format!("Added '{}' ({})", item.name, format_paise(item.price_paise)),
                ToastType::Success,
            );
        }
    }

    pub fn increment_selected_cart_item(&mut self) {
        if let Some(i) = self.cart_state.selected() {
            if i < self.cart.len() {
                self.cart[i].quantity = self.cart[i].quantity.saturating_add(1);
                self.set_toast(
                    format!("Qty increased for '{}' to {}", self.cart[i].name, self.cart[i].quantity),
                    ToastType::Success,
                );
            }
        }
    }

    pub fn decrement_selected_cart_item(&mut self) {
        if let Some(i) = self.cart_state.selected() {
            if i < self.cart.len() {
                if self.cart[i].quantity > 1 {
                    self.cart[i].quantity -= 1;
                    self.set_toast(
                        format!("Qty decreased for '{}' to {}", self.cart[i].name, self.cart[i].quantity),
                        ToastType::Info,
                    );
                } else {
                    let removed = self.cart.remove(i);
                    self.set_toast(format!("Removed '{}' from cart", removed.name), ToastType::Info);
                    if self.cart.is_empty() {
                        self.cart_state.select(None);
                        self.active_pane = ActivePane::Menu;
                    } else if i >= self.cart.len() {
                        self.cart_state.select(Some(self.cart.len() - 1));
                    }
                }
            }
        }
    }

    pub fn delete_selected_cart_item(&mut self) {
        if let Some(i) = self.cart_state.selected() {
            if i < self.cart.len() {
                let removed = self.cart.remove(i);
                self.set_toast(format!("Deleted '{}'", removed.name), ToastType::Info);
                if self.cart.is_empty() {
                    self.cart_state.select(None);
                    self.active_pane = ActivePane::Menu;
                } else if i >= self.cart.len() {
                    self.cart_state.select(Some(self.cart.len() - 1));
                }
            }
        }
    }

    pub fn clear_cart(&mut self) {
        if self.cart.is_empty() {
            self.set_toast("Cart is already empty", ToastType::Info);
        } else {
            self.cart.clear();
            self.cart_state.select(None);
            self.active_pane = ActivePane::Menu;
            self.set_toast("Cart cleared", ToastType::Info);
        }
    }

    // --- On-the-Go Price Alteration & Custom Making Charges ---

    pub fn start_editing_selected_price(&mut self) {
        if let Some(i) = self.cart_state.selected() {
            if let Some(item) = self.cart.get(i) {
                // Pre-fill buffer with existing rupees (e.g. "130.00")
                let rupees = item.price_paise as f64 / 100.0;
                self.input_buffer = format!("{:.2}", rupees);
                self.input_mode = InputMode::EditingPrice;
            }
        } else {
            self.set_toast("Select an item in Cart to edit price", ToastType::Error);
        }
    }

    pub fn commit_edited_price(&mut self) {
        match parse_rupees_to_paise(&self.input_buffer) {
            Ok(new_paise) => {
                if let Some(i) = self.cart_state.selected() {
                    if let Some(item) = self.cart.get_mut(i) {
                        item.price_paise = new_paise;
                        let name = item.name.clone();
                        self.set_toast(
                            format!("Updated price of '{}' to {}", name, format_paise(new_paise)),
                            ToastType::Success,
                        );
                    }
                }
                self.input_buffer.clear();
                self.input_mode = InputMode::Normal;
            }
            Err(err) => {
                self.set_toast(format!("Invalid price: {err}"), ToastType::Error);
            }
        }
    }

    pub fn start_custom_item(&mut self) {
        self.input_buffer.clear();
        self.pending_custom_name.clear();
        self.input_mode = InputMode::AddingCustomName;
    }

    pub fn commit_custom_name(&mut self) {
        let trimmed = self.input_buffer.trim().to_string();
        if trimmed.is_empty() {
            self.set_toast("Item description cannot be empty", ToastType::Error);
            return;
        }
        self.pending_custom_name = trimmed;
        self.input_buffer.clear();
        self.input_mode = InputMode::AddingCustomPrice;
    }

    pub fn commit_custom_price(&mut self) {
        match parse_rupees_to_paise(&self.input_buffer) {
            Ok(price_paise) => {
                let name = std::mem::take(&mut self.pending_custom_name);
                let custom_item = CartItem::new_custom("CUSTOM", name.clone(), price_paise, 1);
                self.cart.push(custom_item);
                let pos = self.cart.len() - 1;
                self.cart_state.select(Some(pos));
                self.active_pane = ActivePane::Cart;
                self.set_toast(
                    format!("Added custom item '{}' ({})", name, format_paise(price_paise)),
                    ToastType::Success,
                );
                self.input_buffer.clear();
                self.input_mode = InputMode::Normal;
            }
            Err(err) => {
                self.set_toast(format!("Invalid price: {err}"), ToastType::Error);
            }
        }
    }

    // --- Settle & Print Bill ---

    pub fn settle_and_print(&mut self) -> Result<Option<BillRecord>, DbError> {
        if self.cart.is_empty() {
            self.set_toast("Cart is empty! Add items before checkout.", ToastType::Error);
            return Ok(None);
        }

        let summary = BillSummary::calculate(&self.cart);
        match self.db.save_bill(&self.cart, &summary) {
            Ok(record) => {
                let msg = format!(
                    "✓ Bill #{} settled ({})! Saved to receipt.txt",
                    record.id,
                    format_paise(record.summary.total_paise)
                );
                self.cart.clear();
                self.cart_state.select(None);
                self.active_pane = ActivePane::Menu;
                self.set_toast(msg, ToastType::Success);
                Ok(Some(record))
            }
            Err(err) => {
                tracing::error!(%err, "Error settling bill");
                self.set_toast(format!("Failed to settle: {err}"), ToastType::Error);
                Err(err)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn setup_test_app() -> App {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("app_test.redb");
        let db = Database::open(&db_path).unwrap();
        App::new(db).unwrap()
    }

    #[test]
    fn test_app_add_item_by_code() {
        let mut app = setup_test_app();
        assert!(app.cart.is_empty());

        // Add 101 (Dal Tadka)
        assert!(app.add_item_by_code("101"));
        assert_eq!(app.cart.len(), 1);
        assert_eq!(app.cart[0].name, "Dal Tadka");
        assert_eq!(app.cart[0].quantity, 1);

        // Add 101 again -> increments quantity
        assert!(app.add_item_by_code("101"));
        assert_eq!(app.cart.len(), 1);
        assert_eq!(app.cart[0].quantity, 2);

        // Add non-existent code
        assert!(!app.add_item_by_code("999"));
    }

    #[test]
    fn test_dhaba_mutton_making_workflow() {
        let mut app = setup_test_app();

        // Customer brings 1kg mutton:
        // Dish 1: 1/2kg Mutton Masala (Making) at custom negotiated price ₹180.00
        app.input_buffer = "1/2kg Mutton Masala (Making)".to_string();
        app.commit_custom_name();
        assert_eq!(app.input_mode, InputMode::AddingCustomPrice);
        app.input_buffer = "180.00".to_string();
        app.commit_custom_price();
        assert_eq!(app.input_mode, InputMode::Normal);
        assert_eq!(app.cart.len(), 1);
        assert_eq!(app.cart[0].price_paise, 18000);

        // Dish 2: 1/2kg Mutton Fry (Making) at custom price ₹160.00
        app.input_buffer = "1/2kg Mutton Fry (Making)".to_string();
        app.commit_custom_name();
        app.input_buffer = "160".to_string();
        app.commit_custom_price();
        assert_eq!(app.cart.len(), 2);
        assert_eq!(app.cart[1].price_paise, 16000);

        // Add 6 Tandoori Rotis (Code 201)
        assert!(app.add_item_by_code("201"));
        // Set quantity to 6
        app.cart.last_mut().unwrap().quantity = 6;

        // Alter price of Mutton Fry on the go to ₹170.00
        app.cart_state.select(Some(1));
        app.start_editing_selected_price();
        assert_eq!(app.input_mode, InputMode::EditingPrice);
        app.input_buffer = "170.00".to_string();
        app.commit_edited_price();
        assert_eq!(app.cart[1].price_paise, 17000);

        // Verify summary
        // 18000 + 17000 + 6*1500 (9000) = 44000 paise (₹440.00)
        let summary = BillSummary::calculate(&app.cart);
        assert_eq!(summary.subtotal_paise, 44000);
        assert_eq!(summary.cgst_paise, 1100); // 2.5% = ₹11.00
        assert_eq!(summary.sgst_paise, 1100); // 2.5% = ₹11.00
        assert_eq!(summary.total_paise, 46200); // ₹462.00

        // Settle & Print
        let bill = app.settle_and_print().unwrap().unwrap();
        assert_eq!(bill.summary.total_paise, 46200);
        assert!(app.cart.is_empty());
    }

    #[test]
    fn test_cart_quantity_adjustments() {
        let mut app = setup_test_app();
        app.add_item_by_code("101");
        app.cart_state.select(Some(0));

        app.increment_selected_cart_item();
        assert_eq!(app.cart[0].quantity, 2);

        app.decrement_selected_cart_item();
        assert_eq!(app.cart[0].quantity, 1);

        // Decrementing when quantity == 1 removes item
        app.decrement_selected_cart_item();
        assert!(app.cart.is_empty());
    }
}
