use crate::models::{format_paise, BillRecord, BillSummary, CartItem, MenuItem};
use chrono::Local;
use redb::{Database as RedbDatabase, ReadableTable, ReadableTableMetadata, TableDefinition};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use thiserror::Error;

pub const MENU_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("menu_items");
pub const BILLS_TABLE: TableDefinition<u64, &[u8]> = TableDefinition::new("bills");
pub const META_TABLE: TableDefinition<&str, u64> = TableDefinition::new("metadata");

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Database error: {0}")]
    Redb(#[from] Box<redb::Error>),
    #[error("Transaction error: {0}")]
    Transaction(#[from] Box<redb::TransactionError>),
    #[error("Table error: {0}")]
    Table(#[from] Box<redb::TableError>),
    #[error("Database open error: {0}")]
    DatabaseError(#[from] Box<redb::DatabaseError>),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Storage error: {0}")]
    Storage(#[from] Box<redb::StorageError>),
    #[error("Commit error: {0}")]
    Commit(#[from] Box<redb::CommitError>),
}

impl From<redb::Error> for DbError {
    fn from(err: redb::Error) -> Self {
        DbError::Redb(Box::new(err))
    }
}

impl From<redb::TransactionError> for DbError {
    fn from(err: redb::TransactionError) -> Self {
        DbError::Transaction(Box::new(err))
    }
}

impl From<redb::TableError> for DbError {
    fn from(err: redb::TableError) -> Self {
        DbError::Table(Box::new(err))
    }
}

impl From<redb::DatabaseError> for DbError {
    fn from(err: redb::DatabaseError) -> Self {
        DbError::DatabaseError(Box::new(err))
    }
}

impl From<redb::StorageError> for DbError {
    fn from(err: redb::StorageError) -> Self {
        DbError::Storage(Box::new(err))
    }
}

impl From<redb::CommitError> for DbError {
    fn from(err: redb::CommitError) -> Self {
        DbError::Commit(Box::new(err))
    }
}

#[derive(Clone)]
pub struct Database {
    db: Arc<RedbDatabase>,
    base_dir: PathBuf,
}

impl Database {
    /// Opens or creates the redb database at the specified path.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, DbError> {
        let path_ref = path.as_ref();
        let base_dir = path_ref
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf();

        if let Some(parent) = path_ref.parent() {
            fs::create_dir_all(parent)?;
        }

        let db = if path_ref.exists() {
            tracing::info!(path = %path_ref.display(), "Opening existing redb database");
            RedbDatabase::open(path_ref)?
        } else {
            tracing::info!(path = %path_ref.display(), "Creating new redb database");
            RedbDatabase::create(path_ref)?
        };

        let db_instance = Self {
            db: Arc::new(db),
            base_dir,
        };

        // Initialize tables and seed default menu if empty
        db_instance.init_tables_and_seeds()?;

        Ok(db_instance)
    }

    /// Initializes tables and seeds default items if menu is empty.
    fn init_tables_and_seeds(&self) -> Result<(), DbError> {
        let write_txn = self.db.begin_write()?;
        let is_menu_empty = {
            let menu_table = write_txn.open_table(MENU_TABLE)?;
            let mut meta_table = write_txn.open_table(META_TABLE)?;
            let _bills_table = write_txn.open_table(BILLS_TABLE)?;

            // Initialize next_bill_id counter if missing
            if meta_table.get("next_bill_id")?.is_none() {
                meta_table.insert("next_bill_id", 101u64)?;
                tracing::info!("Initialized next_bill_id counter at 101");
            }

            menu_table.is_empty()?
        };

        if is_menu_empty {
            tracing::info!("Seeding initial dhaba menu catalog into redb");
            let mut menu_table = write_txn.open_table(MENU_TABLE)?;
            let seeds = Self::default_seeds();
            for item in seeds {
                let serialized = serde_json::to_vec(&item)?;
                menu_table.insert(item.code.as_str(), serialized.as_slice())?;
            }
        }

        write_txn.commit()?;
        Ok(())
    }

    /// Default seed data representing typical North/West Indian dhaba offerings,
    /// including customer-supplied meat/fish cooking ("Making Charges").
    pub fn default_seeds() -> Vec<MenuItem> {
        vec![
            // Mains
            MenuItem::new("101", "Dal Tadka", "Mains", 13000),
            MenuItem::new("102", "Dal Makhani", "Mains", 16000),
            MenuItem::new("103", "Paneer Butter Masala", "Mains", 21000),
            MenuItem::new("104", "Kadhai Paneer", "Mains", 22000),
            MenuItem::new("105", "Palak Paneer", "Mains", 20000),
            MenuItem::new("106", "Sev Tamatar", "Mains", 14000),
            MenuItem::new("107", "Chana Masala", "Mains", 15000),
            MenuItem::new("108", "Egg Curry (2 Eggs)", "Mains", 13000),
            MenuItem::new("109", "Chicken Masala Dhaba Style", "Mains", 24000),
            MenuItem::new("110", "Dhaba Mutton Handi", "Mains", 32000),
            // Breads
            MenuItem::new("201", "Tandoori Roti", "Breads", 1500),
            MenuItem::new("202", "Butter Roti", "Breads", 2000),
            MenuItem::new("203", "Plain Naan", "Breads", 3500),
            MenuItem::new("204", "Butter Naan", "Breads", 4500),
            MenuItem::new("205", "Garlic Naan", "Breads", 6000),
            MenuItem::new("206", "Laccha Paratha", "Breads", 5000),
            // Rice & Biryani
            MenuItem::new("301", "Steamed Basmati Rice", "Rice", 10000),
            MenuItem::new("302", "Jeera Rice", "Rice", 12000),
            MenuItem::new("303", "Veg Pulao", "Rice", 15000),
            MenuItem::new("304", "Veg Dum Biryani", "Rice", 19000),
            MenuItem::new("305", "Chicken Dum Biryani", "Rice", 25000),
            // Making Charges (Customer-supplied raw meat/poultry)
            MenuItem::new("401", "Mutton Making (1/2 kg)", "Making Chg", 18000),
            MenuItem::new("402", "Mutton Making (1 kg)", "Making Chg", 32000),
            MenuItem::new("403", "Chicken Making (1/2 kg)", "Making Chg", 13000),
            MenuItem::new("404", "Chicken Making (1 kg)", "Making Chg", 24000),
            MenuItem::new("405", "Fish Making (1 kg)", "Making Chg", 20000),
            MenuItem::new("406", "Custom Cooking Chg", "Making Chg", 10000),
            // Beverages & Sides
            MenuItem::new("501", "Sweet Lassi", "Beverages", 6000),
            MenuItem::new("502", "Chaas / Buttermilk", "Beverages", 3000),
            MenuItem::new("503", "Special Masala Chai", "Beverages", 2000),
            MenuItem::new("504", "Roasted Papad", "Sides", 1500),
            MenuItem::new("505", "Masala Papad", "Sides", 3500),
            MenuItem::new("506", "Green Salad", "Sides", 4000),
        ]
    }

    /// Retrieves all menu items ordered by code.
    pub fn get_menu(&self) -> Result<Vec<MenuItem>, DbError> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(MENU_TABLE)?;

        let mut items = Vec::new();
        for row in table.iter()? {
            let (_key, val) = row?;
            let item: MenuItem = serde_json::from_slice(val.value())?;
            items.push(item);
        }

        items.sort_by(|a, b| a.code.cmp(&b.code));
        Ok(items)
    }

    /// Retrieves the most recent N bills from the database.
    pub fn get_recent_bills(&self, limit: usize) -> Result<Vec<BillRecord>, DbError> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(BILLS_TABLE)?;

        let mut bills = Vec::new();
        for row in table.iter()? {
            let (_key, val) = row?;
            let record: BillRecord = serde_json::from_slice(val.value())?;
            bills.push(record);
        }

        bills.sort_by_key(|a| std::cmp::Reverse(a.id));
        bills.truncate(limit);
        Ok(bills)
    }

    /// Atomically records a finalized bill into `redb` and exports a receipt file.
    pub fn save_bill(
        &self,
        table_id: &str,
        items: &[CartItem],
        summary: &BillSummary,
    ) -> Result<BillRecord, DbError> {
        let write_txn = self.db.begin_write()?;
        let bill_id;
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        {
            let mut meta_table = write_txn.open_table(META_TABLE)?;
            let current_id = meta_table
                .get("next_bill_id")?
                .map(|val| val.value())
                .unwrap_or(101);

            bill_id = current_id;
            meta_table.insert("next_bill_id", current_id + 1)?;

            let mut bills_table = write_txn.open_table(BILLS_TABLE)?;
            let record = BillRecord {
                id: bill_id,
                table_id: table_id.to_string(),
                timestamp: timestamp.clone(),
                items: items.to_vec(),
                summary: summary.clone(),
            };

            let serialized = serde_json::to_vec(&record)?;
            bills_table.insert(bill_id, serialized.as_slice())?;
        }

        write_txn.commit()?;
        tracing::info!(
            bill_id,
            table = table_id,
            total = format_paise(summary.total_paise),
            "Bill committed to redb"
        );

        let record = BillRecord {
            id: bill_id,
            table_id: table_id.to_string(),
            timestamp,
            items: items.to_vec(),
            summary: summary.clone(),
        };

        // Export physical receipt files
        if let Err(err) = self.export_receipt(&record) {
            tracing::error!(bill_id, %err, "Failed to export receipt file");
        }

        Ok(record)
    }

    /// Exports an itemized text receipt to `receipt.txt` (latest)
    /// and archives a copy to `receipts/bill_{id}.txt`.
    pub fn export_receipt(&self, bill: &BillRecord) -> Result<String, std::io::Error> {
        let mut buffer = String::new();
        buffer.push_str("============================================================\n");
        buffer.push_str("                      DHABA TUI BILL\n");
        buffer.push_str("============================================================\n");
        buffer.push_str(&format!(
            "Bill No: #{:<16} Table: {:<8} Date: {}\n",
            bill.id, bill.table_id, bill.timestamp
        ));
        buffer.push_str("------------------------------------------------------------\n");
        buffer.push_str("Item                            Qty    Price        Total\n");
        buffer.push_str("------------------------------------------------------------\n");

        for item in &bill.items {
            let item_display = if item.is_custom {
                format!("* {}", item.name)
            } else {
                item.name.clone()
            };
            buffer.push_str(&format!(
                "{:<30} {:>3} {:>8} {:>12}\n",
                if item_display.len() > 30 {
                    &item_display[..30]
                } else {
                    &item_display
                },
                item.quantity,
                format_paise(item.price_paise),
                format_paise(item.line_total_paise())
            ));
        }

        buffer.push_str("------------------------------------------------------------\n");
        buffer.push_str(&format!(
            "Total Items: {} ({} lines)\n",
            bill.summary.total_items, bill.summary.total_lines
        ));
        buffer.push_str(&format!(
            "Subtotal:                                       {:>12}\n",
            format_paise(bill.summary.subtotal_paise)
        ));
        buffer.push_str(&format!(
            "CGST (2.5%):                                    {:>12}\n",
            format_paise(bill.summary.cgst_paise)
        ));
        buffer.push_str(&format!(
            "SGST (2.5%):                                    {:>12}\n",
            format_paise(bill.summary.sgst_paise)
        ));
        buffer.push_str("------------------------------------------------------------\n");
        buffer.push_str(&format!(
            "GRAND TOTAL:                                    {:>12}\n",
            format_paise(bill.summary.total_paise)
        ));
        buffer.push_str("============================================================\n");
        buffer.push_str("                 Thank You! Visit Again!\n");
        buffer.push_str("============================================================\n");

        // Write to latest receipt.txt
        let latest_path = self.base_dir.join("receipt.txt");
        let mut file = File::create(&latest_path)?;
        file.write_all(buffer.as_bytes())?;

        // Archive into receipts/ directory
        let receipts_dir = self.base_dir.join("receipts");
        let _ = fs::create_dir_all(&receipts_dir);
        let archive_path = receipts_dir.join(format!("bill_{}.txt", bill.id));
        if let Ok(mut archive_file) = File::create(&archive_path) {
            let _ = archive_file.write_all(buffer.as_bytes());
        }

        tracing::info!(path = %latest_path.display(), "Exported latest receipt");
        Ok(buffer)
    }

    /// Exports an itemized text receipt marked as a duplicate to `receipt.txt`.
    pub fn export_duplicate_receipt(&self, bill: &BillRecord) -> Result<String, std::io::Error> {
        let mut buffer = String::new();
        buffer.push_str("============================================================\n");
        buffer.push_str("          DHABA TUI BILL — *** DUPLICATE RECEIPT ***        \n");
        buffer.push_str("============================================================\n");
        buffer.push_str(&format!(
            "Bill No: #{:<16} Table: {:<8} Date: {}\n",
            bill.id, bill.table_id, bill.timestamp
        ));
        buffer.push_str("------------------------------------------------------------\n");
        buffer.push_str("Item                            Qty    Price        Total\n");
        buffer.push_str("------------------------------------------------------------\n");

        for item in &bill.items {
            let item_display = if item.is_custom {
                format!("* {}", item.name)
            } else {
                item.name.clone()
            };
            buffer.push_str(&format!(
                "{:<30} {:>3} {:>8} {:>12}\n",
                if item_display.len() > 30 {
                    &item_display[..30]
                } else {
                    &item_display
                },
                item.quantity,
                format_paise(item.price_paise),
                format_paise(item.line_total_paise())
            ));
        }

        buffer.push_str("------------------------------------------------------------\n");
        buffer.push_str(&format!(
            "Total Items: {} ({} lines)\n",
            bill.summary.total_items, bill.summary.total_lines
        ));
        buffer.push_str(&format!(
            "Subtotal:                                       {:>12}\n",
            format_paise(bill.summary.subtotal_paise)
        ));
        buffer.push_str(&format!(
            "CGST (2.5%):                                    {:>12}\n",
            format_paise(bill.summary.cgst_paise)
        ));
        buffer.push_str(&format!(
            "SGST (2.5%):                                    {:>12}\n",
            format_paise(bill.summary.sgst_paise)
        ));
        buffer.push_str("------------------------------------------------------------\n");
        buffer.push_str(&format!(
            "GRAND TOTAL:                                    {:>12}\n",
            format_paise(bill.summary.total_paise)
        ));
        buffer.push_str("============================================================\n");
        buffer.push_str("           *** DUPLICATE COPY / REPRINT ***                 \n");
        buffer.push_str("============================================================\n");

        let latest_path = self.base_dir.join("receipt.txt");
        let mut file = File::create(&latest_path)?;
        file.write_all(buffer.as_bytes())?;

        tracing::info!(bill_id = bill.id, "Exported duplicate receipt");
        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_initialization_and_seed() {
        let dir = tempfile::tempdir().unwrap();
        let db_path = dir.path().join("test.redb");
        let db = Database::open(&db_path).unwrap();

        let menu = db.get_menu().unwrap();
        assert!(!menu.is_empty());
        assert!(menu
            .iter()
            .any(|m| m.code == "101" && m.name == "Dal Tadka"));
        assert!(menu
            .iter()
            .any(|m| m.code == "401" && m.name.contains("Mutton Making")));
    }

    #[test]
    fn test_save_bill_transaction() {
        let dir = tempfile::tempdir().unwrap();
        let db_path = dir.path().join("test_bill.redb");
        let db = Database::open(&db_path).unwrap();

        let items = vec![
            CartItem {
                code: "401".to_string(),
                name: "1/2kg Mutton Masala (Making)".to_string(),
                price_paise: 18000,
                quantity: 1,
                is_custom: true,
            },
            CartItem {
                code: "201".to_string(),
                name: "Tandoori Roti".to_string(),
                price_paise: 1500,
                quantity: 4,
                is_custom: false,
            },
        ];

        let summary = BillSummary::calculate(&items);
        let bill = db.save_bill("T1", &items, &summary).unwrap();

        assert_eq!(bill.id, 101);
        assert_eq!(bill.table_id, "T1");
        assert_eq!(bill.summary.subtotal_paise, 24000); // 18000 + 4*1500 = 24000
        assert_eq!(bill.items.len(), 2);

        // Second bill should have incremented ID
        let bill2 = db.save_bill("T2", &items, &summary).unwrap();
        assert_eq!(bill2.id, 102);
        assert_eq!(bill2.table_id, "T2");

        // Check receipt file creation
        let receipt_path = dir.path().join("receipt.txt");
        assert!(receipt_path.exists());

        // Test duplicate receipt reprint
        let dup = db.export_duplicate_receipt(&bill).unwrap();
        assert!(dup.contains("*** DUPLICATE RECEIPT ***"));
        assert!(dup.contains("Bill No: #101"));
    }
}
