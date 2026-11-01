use crate::db::{Database, DbError};
use crate::models::{
    format_paise, parse_rupees_to_paise, BillRecord, CartItem, MenuItem, TableOrder, TableStatus,
};
use crate::web::{DashboardData, WebServerHandle};
use chrono::Local;
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
    pub tables: Vec<TableOrder>,
    pub active_table_idx: usize,
    pub cart_state: TableState,
    pub active_pane: ActivePane,
    pub input_mode: InputMode,
    pub quick_code_buffer: String,
    pub input_buffer: String,
    pub pending_custom_name: String,
    pub toast: Option<(String, Instant, ToastType)>,
    pub should_quit: bool,
    pub web_handle: Option<WebServerHandle>,
    pub recent_bills: Vec<BillRecord>,
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

        let tables = vec![
            TableOrder::new("T1", "Table 1"),
            TableOrder::new("T2", "Table 2"),
            TableOrder::new("T3", "Table 3"),
            TableOrder::new("T4", "Table 4"),
            TableOrder::new("T5", "Table 5"),
            TableOrder::new("T6", "Table 6"),
            TableOrder::new("T7", "Table 7"),
            TableOrder::new("T8", "Table 8"),
            TableOrder::new("Parcel", "Parcel / Takeaway"),
        ];

        let recent_bills = db.get_recent_bills(10).unwrap_or_default();

        let mut app = Self {
            db,
            menu,
            menu_state,
            tables,
            active_table_idx: 0,
            cart_state,
            active_pane: ActivePane::Menu,
            input_mode: InputMode::Normal,
            quick_code_buffer: String::new(),
            input_buffer: String::new(),
            pending_custom_name: String::new(),
            toast: Some((
                "Welcome to DhabaTUI! Press 't' to switch tables, 'p' to settle.".to_string(),
                Instant::now(),
                ToastType::Info,
            )),
            should_quit: false,
            web_handle: None,
            recent_bills,
        };

        // Start LAN web server
        let initial_data = DashboardData::build(&app.tables, &app.recent_bills, "Detecting...");
        match crate::web::start_lan_server(initial_data) {
            Ok(handle) => {
                let url = handle.lan_url.clone();
                app.web_handle = Some(handle);
                app.sync_web_dashboard();
                app.set_toast(format!("Live LAN Web Dashboard: {url}"), ToastType::Info);
            }
            Err(err) => {
                tracing::error!(%err, "Could not start LAN web server");
            }
        }

        Ok(app)
    }

    pub fn lan_url(&self) -> Option<&str> {
        self.web_handle.as_ref().map(|h| h.lan_url.as_str())
    }

    pub fn sync_web_dashboard(&self) {
        if let Some(handle) = &self.web_handle {
            let data = DashboardData::build(&self.tables, &self.recent_bills, &handle.lan_url);
            if let Ok(mut lock) = handle.shared_data.write() {
                *lock = data;
            }
        }
    }

    pub fn active_table(&self) -> &TableOrder {
        &self.tables[self.active_table_idx]
    }

    pub fn active_table_mut(&mut self) -> &mut TableOrder {
        &mut self.tables[self.active_table_idx]
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

    // --- Table Navigation ---

    pub fn next_table(&mut self) {
        if self.tables.is_empty() {
            return;
        }
        self.active_table_idx = (self.active_table_idx + 1) % self.tables.len();
        self.on_active_table_changed();
    }

    pub fn previous_table(&mut self) {
        if self.tables.is_empty() {
            return;
        }
        if self.active_table_idx == 0 {
            self.active_table_idx = self.tables.len() - 1;
        } else {
            self.active_table_idx -= 1;
        }
        self.on_active_table_changed();
    }

    pub fn set_table_by_idx(&mut self, idx: usize) {
        if idx < self.tables.len() {
            self.active_table_idx = idx;
            self.on_active_table_changed();
        }
    }

    fn on_active_table_changed(&mut self) {
        let items_len = self.active_table().items.len();
        if items_len > 0 {
            self.cart_state.select(Some(0));
        } else {
            self.cart_state.select(None);
            self.active_pane = ActivePane::Menu;
        }
        let label = self.active_table().label.clone();
        let status = self.active_table().status.as_str();
        self.set_toast(format!("Switched to {label} [{status}]"), ToastType::Info);
    }

    pub fn toggle_active_table_status(&mut self) {
        let table = self.active_table_mut();
        table.status = match table.status {
            TableStatus::Vacant => TableStatus::Ordering,
            TableStatus::Ordering => TableStatus::Served,
            TableStatus::Served => TableStatus::Billed,
            TableStatus::Billed => TableStatus::Ordering,
        };
        let new_status = table.status.as_str();
        let label = table.label.clone();
        self.set_toast(
            format!("{label} status set to {new_status}"),
            ToastType::Success,
        );
        self.sync_web_dashboard();
    }

    // --- Pane & Item Navigation ---

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
                let items_len = self.active_table().items.len();
                if items_len == 0 {
                    return;
                }
                let i = match self.cart_state.selected() {
                    Some(i) => {
                        if i >= items_len - 1 {
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
                let items_len = self.active_table().items.len();
                if items_len == 0 {
                    return;
                }
                let i = match self.cart_state.selected() {
                    Some(i) => {
                        if i == 0 {
                            items_len - 1
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
                if !self.active_table().items.is_empty() && self.cart_state.selected().is_none() {
                    self.cart_state.select(Some(0));
                }
                ActivePane::Cart
            }
            ActivePane::Cart => ActivePane::Menu,
        };
    }

    // --- Cart Actions on Active Table ---

    pub fn add_selected_menu_item(&mut self) {
        if let Some(i) = self.menu_state.selected() {
            if let Some(menu_item) = self.menu.get(i).cloned() {
                self.add_or_increment_menu_item(&menu_item);
            }
        }
    }

    pub fn add_item_by_code(&mut self, code: &str) -> bool {
        let trimmed = code.trim();
        if let Some(item) = self
            .menu
            .iter()
            .find(|m| m.code.eq_ignore_ascii_case(trimmed))
            .cloned()
        {
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
        let table = &mut self.tables[self.active_table_idx];
        if table.status == TableStatus::Vacant {
            table.status = TableStatus::Ordering;
            table.opened_at = Some(Local::now().format("%H:%M").to_string());
        }

        let table_label = table.label.clone();
        if let Some(pos) = table
            .items
            .iter()
            .position(|c| !c.is_custom && c.code == item.code && c.price_paise == item.price_paise)
        {
            table.items[pos].quantity = table.items[pos].quantity.saturating_add(1);
            let name = table.items[pos].name.clone();
            let qty = table.items[pos].quantity;
            self.cart_state.select(Some(pos));
            self.set_toast(
                format!("{table_label}: Incremented '{name}' (Qty: {qty})"),
                ToastType::Success,
            );
        } else {
            let cart_item = CartItem::from_menu(item, 1);
            table.items.push(cart_item);
            let new_pos = table.items.len() - 1;
            self.cart_state.select(Some(new_pos));
            self.set_toast(
                format!(
                    "{table_label}: Added '{}' ({})",
                    item.name,
                    format_paise(item.price_paise)
                ),
                ToastType::Success,
            );
        }

        self.sync_web_dashboard();
    }

    pub fn increment_selected_cart_item(&mut self) {
        let selected = self.cart_state.selected();
        if let Some(i) = selected {
            let table = &mut self.tables[self.active_table_idx];
            if i < table.items.len() {
                table.items[i].quantity = table.items[i].quantity.saturating_add(1);
                let name = table.items[i].name.clone();
                let qty = table.items[i].quantity;
                self.set_toast(
                    format!("Qty increased for '{name}' to {qty}"),
                    ToastType::Success,
                );
                self.sync_web_dashboard();
            }
        }
    }

    pub fn decrement_selected_cart_item(&mut self) {
        let selected = self.cart_state.selected();
        if let Some(i) = selected {
            let table = &mut self.tables[self.active_table_idx];
            if i < table.items.len() {
                if table.items[i].quantity > 1 {
                    table.items[i].quantity -= 1;
                    let name = table.items[i].name.clone();
                    let qty = table.items[i].quantity;
                    self.set_toast(
                        format!("Qty decreased for '{name}' to {qty}"),
                        ToastType::Info,
                    );
                } else {
                    let removed = table.items.remove(i);
                    let is_empty = table.items.is_empty();
                    let remaining_len = table.items.len();
                    if is_empty {
                        table.status = TableStatus::Vacant;
                        table.opened_at = None;
                        self.cart_state.select(None);
                        self.active_pane = ActivePane::Menu;
                    } else if i >= remaining_len {
                        self.cart_state.select(Some(remaining_len - 1));
                    }
                    self.set_toast(
                        format!("Removed '{}' from order", removed.name),
                        ToastType::Info,
                    );
                }
                self.sync_web_dashboard();
            }
        }
    }

    pub fn delete_selected_cart_item(&mut self) {
        let selected = self.cart_state.selected();
        if let Some(i) = selected {
            let table = &mut self.tables[self.active_table_idx];
            if i < table.items.len() {
                let removed = table.items.remove(i);
                let is_empty = table.items.is_empty();
                let remaining_len = table.items.len();
                if is_empty {
                    table.status = TableStatus::Vacant;
                    table.opened_at = None;
                    self.cart_state.select(None);
                    self.active_pane = ActivePane::Menu;
                } else if i >= remaining_len {
                    self.cart_state.select(Some(remaining_len - 1));
                }
                self.set_toast(format!("Deleted '{}'", removed.name), ToastType::Info);
                self.sync_web_dashboard();
            }
        }
    }

    pub fn clear_cart(&mut self) {
        let table = &mut self.tables[self.active_table_idx];
        if table.items.is_empty() {
            self.set_toast("Table order is already empty", ToastType::Info);
        } else {
            table.items.clear();
            table.status = TableStatus::Vacant;
            table.opened_at = None;
            self.cart_state.select(None);
            self.active_pane = ActivePane::Menu;
            self.set_toast("Table order cleared", ToastType::Info);
            self.sync_web_dashboard();
        }
    }

    // --- On-the-Go Price Alteration & Custom Making Charges ---

    pub fn start_editing_selected_price(&mut self) {
        let table = self.active_table();
        if let Some(i) = self.cart_state.selected() {
            if let Some(item) = table.items.get(i) {
                let rupees = item.price_paise as f64 / 100.0;
                self.input_buffer = format!("{:.2}", rupees);
                self.input_mode = InputMode::EditingPrice;
            }
        } else {
            self.set_toast("Select an item in order to edit price", ToastType::Error);
        }
    }

    pub fn commit_edited_price(&mut self) {
        match parse_rupees_to_paise(&self.input_buffer) {
            Ok(new_paise) => {
                let selected = self.cart_state.selected();
                if let Some(i) = selected {
                    let table = &mut self.tables[self.active_table_idx];
                    if let Some(item) = table.items.get_mut(i) {
                        item.price_paise = new_paise;
                        let name = item.name.clone();
                        self.set_toast(
                            format!("Updated price of '{name}' to {}", format_paise(new_paise)),
                            ToastType::Success,
                        );
                    }
                }
                self.input_buffer.clear();
                self.input_mode = InputMode::Normal;
                self.sync_web_dashboard();
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
                let table = &mut self.tables[self.active_table_idx];
                if table.status == TableStatus::Vacant {
                    table.status = TableStatus::Ordering;
                    table.opened_at = Some(Local::now().format("%H:%M").to_string());
                }

                table.items.push(custom_item);
                let pos = table.items.len() - 1;
                self.cart_state.select(Some(pos));
                self.active_pane = ActivePane::Cart;
                self.set_toast(
                    format!("Added custom item '{name}' ({})", format_paise(price_paise)),
                    ToastType::Success,
                );
                self.input_buffer.clear();
                self.input_mode = InputMode::Normal;
                self.sync_web_dashboard();
            }
            Err(err) => {
                self.set_toast(format!("Invalid price: {err}"), ToastType::Error);
            }
        }
    }

    // --- Settle & Print Bill ---

    pub fn settle_and_print(&mut self) -> Result<Option<BillRecord>, DbError> {
        let table = self.active_table();
        if table.items.is_empty() {
            self.set_toast(
                format!(
                    "{}: Order is empty! Add items before checkout.",
                    table.label
                ),
                ToastType::Error,
            );
            return Ok(None);
        }

        let summary = table.summary();
        let table_id = table.id.clone();
        let table_label = table.label.clone();

        match self.db.save_bill(&table_id, &table.items, &summary) {
            Ok(record) => {
                let msg = format!(
                    "✓ Bill #{} settled for {} ({})! Saved to receipt.txt",
                    record.id,
                    table_label,
                    format_paise(record.summary.total_paise)
                );

                // Add to recent bills history
                self.recent_bills.insert(0, record.clone());
                self.recent_bills.truncate(10);

                // Reset table
                let table = &mut self.tables[self.active_table_idx];
                table.items.clear();
                table.status = TableStatus::Vacant;
                table.opened_at = None;

                self.cart_state.select(None);
                self.active_pane = ActivePane::Menu;
                self.set_toast(msg, ToastType::Success);
                self.sync_web_dashboard();
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
        assert!(app.active_table().items.is_empty());

        // Add 101 (Dal Tadka)
        assert!(app.add_item_by_code("101"));
        assert_eq!(app.active_table().items.len(), 1);
        assert_eq!(app.active_table().items[0].name, "Dal Tadka");
        assert_eq!(app.active_table().items[0].quantity, 1);
        assert_eq!(app.active_table().status, TableStatus::Ordering);

        // Add 101 again -> increments quantity
        assert!(app.add_item_by_code("101"));
        assert_eq!(app.active_table().items.len(), 1);
        assert_eq!(app.active_table().items[0].quantity, 2);

        // Add non-existent code
        assert!(!app.add_item_by_code("999"));
    }

    #[test]
    fn test_table_switching() {
        let mut app = setup_test_app();
        assert_eq!(app.active_table().id, "T1");

        // Add item to T1
        app.add_item_by_code("101");
        assert_eq!(app.active_table().items.len(), 1);

        // Switch to T2
        app.next_table();
        assert_eq!(app.active_table().id, "T2");
        assert!(app.active_table().items.is_empty());

        // Add item to T2
        app.add_item_by_code("201");
        assert_eq!(app.active_table().items.len(), 1);
        assert_eq!(app.active_table().items[0].name, "Tandoori Roti");

        // Switch back to T1
        app.previous_table();
        assert_eq!(app.active_table().id, "T1");
        assert_eq!(app.active_table().items[0].name, "Dal Tadka");
    }

    #[test]
    fn test_dhaba_mutton_making_workflow() {
        let mut app = setup_test_app();

        // Customer brings 1kg mutton:
        app.input_buffer = "1/2kg Mutton Masala (Making)".to_string();
        app.commit_custom_name();
        assert_eq!(app.input_mode, InputMode::AddingCustomPrice);
        app.input_buffer = "180.00".to_string();
        app.commit_custom_price();
        assert_eq!(app.input_mode, InputMode::Normal);
        assert_eq!(app.active_table().items.len(), 1);
        assert_eq!(app.active_table().items[0].price_paise, 18000);

        // Dish 2: 1/2kg Mutton Fry (Making) at custom price ₹160.00
        app.input_buffer = "1/2kg Mutton Fry (Making)".to_string();
        app.commit_custom_name();
        app.input_buffer = "160".to_string();
        app.commit_custom_price();
        assert_eq!(app.active_table().items.len(), 2);
        assert_eq!(app.active_table().items[1].price_paise, 16000);

        // Add 6 Tandoori Rotis (Code 201)
        assert!(app.add_item_by_code("201"));
        app.active_table_mut().items.last_mut().unwrap().quantity = 6;

        // Alter price of Mutton Fry on the go to ₹170.00
        app.cart_state.select(Some(1));
        app.start_editing_selected_price();
        assert_eq!(app.input_mode, InputMode::EditingPrice);
        app.input_buffer = "170.00".to_string();
        app.commit_edited_price();
        assert_eq!(app.active_table().items[1].price_paise, 17000);

        // Verify summary
        let summary = app.active_table().summary();
        assert_eq!(summary.subtotal_paise, 44000);
        assert_eq!(summary.cgst_paise, 1100);
        assert_eq!(summary.sgst_paise, 1100);
        assert_eq!(summary.total_paise, 46200);

        // Settle & Print
        let bill = app.settle_and_print().unwrap().unwrap();
        assert_eq!(bill.summary.total_paise, 46200);
        assert!(app.active_table().items.is_empty());
        assert_eq!(app.active_table().status, TableStatus::Vacant);
    }
}
