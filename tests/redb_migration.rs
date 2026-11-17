use dhabatui::db::Database;
use dhabatui::models::{BillRecord, BillSummary, CartItem, MenuItem, PaymentMethod};
use std::fs;

#[test]
fn test_automated_v2_to_v4_migration_end_to_end() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("dhabatui.redb");
    let backup_path = dir.path().join("dhabatui.redb.v2.bak");

    // 1. Synthesize a genuine legacy redb v2 database file
    {
        const LEGACY_MENU: redb_legacy::TableDefinition<&str, &[u8]> =
            redb_legacy::TableDefinition::new("menu_items");
        const LEGACY_BILLS: redb_legacy::TableDefinition<u64, &[u8]> =
            redb_legacy::TableDefinition::new("bills");
        const LEGACY_META: redb_legacy::TableDefinition<&str, u64> =
            redb_legacy::TableDefinition::new("metadata");

        let legacy_db = redb_legacy::Database::create(&db_path).unwrap();
        let write_txn = legacy_db.begin_write().unwrap();
        {
            let mut menu_table = write_txn.open_table(LEGACY_MENU).unwrap();
            let custom_dish = MenuItem::new("999", "Special Highway Tadka", "Mains", 25000);
            let serialized_dish = serde_json::to_vec(&custom_dish).unwrap();
            menu_table
                .insert("999", serialized_dish.as_slice())
                .unwrap();

            let mut meta_table = write_txn.open_table(LEGACY_META).unwrap();
            // Sequential counter at 145 (must not reset to 101)
            meta_table.insert("next_bill_id", 145u64).unwrap();

            let mut bills_table = write_txn.open_table(LEGACY_BILLS).unwrap();
            let sample_item = CartItem {
                code: "999".to_string(),
                name: "Special Highway Tadka".to_string(),
                price_paise: 25000,
                quantity: 2,
                is_custom: false,
            };
            let sample_summary = BillSummary::calculate(std::slice::from_ref(&sample_item));
            let sample_bill = BillRecord {
                id: 144,
                table_id: "T3".to_string(),
                timestamp: "2026-11-17 12:00:00".to_string(),
                items: vec![sample_item],
                summary: sample_summary,
                payment_method: PaymentMethod::Cash,
            };
            let serialized_bill = serde_json::to_vec(&sample_bill).unwrap();
            bills_table
                .insert(144u64, serialized_bill.as_slice())
                .unwrap();
        }
        write_txn.commit().unwrap();
    }

    // Ensure database exists and backup does not yet exist
    assert!(db_path.exists());
    assert!(!backup_path.exists());

    // 2. Open using the upgraded Database::open() — should automatically trigger migration
    let db = Database::open(&db_path).expect("Database::open should succeed and migrate v2 to v4");

    // 3. Verify backup file was created
    assert!(
        backup_path.exists(),
        "Backup file dhabatui.redb.v2.bak must be preserved"
    );
    assert!(
        fs::metadata(&backup_path).unwrap().len() > 0,
        "Backup file must not be empty"
    );

    // 4. Verify data integrity in the migrated database
    // Menu items
    let menu = db.get_menu().expect("get_menu should succeed");
    assert!(
        menu.iter()
            .any(|m| m.code == "999" && m.name == "Special Highway Tadka"),
        "Custom item 'Special Highway Tadka' must be preserved"
    );

    // Historical bills
    let recent = db
        .get_recent_bills(10)
        .expect("get_recent_bills should succeed");
    assert_eq!(recent.len(), 1);
    assert_eq!(recent[0].id, 144);
    assert_eq!(recent[0].table_id, "T3");
    assert_eq!(recent[0].payment_method, PaymentMethod::Cash);

    // Daily report on historical records
    let daily = db
        .get_daily_report("2026-11-17")
        .expect("get_daily_report should succeed");
    assert_eq!(daily.total_bills, 1);
    assert_eq!(daily.total_items_sold, 2);

    // 5. Verify sequential bill counter continuity
    let new_item = CartItem {
        code: "999".to_string(),
        name: "Special Highway Tadka".to_string(),
        price_paise: 25000,
        quantity: 1,
        is_custom: false,
    };
    let new_summary = BillSummary::calculate(std::slice::from_ref(&new_item));
    let new_bill = db
        .save_bill(
            "T4",
            std::slice::from_ref(&new_item),
            &new_summary,
            PaymentMethod::Upi,
        )
        .expect("save_bill should succeed");

    assert_eq!(
        new_bill.id, 145,
        "New bill must continue from next_bill_id (145), not restart at 101"
    );

    let next_bill = db
        .save_bill("T5", &[new_item], &new_summary, PaymentMethod::Cash)
        .expect("second save_bill should succeed");
    assert_eq!(next_bill.id, 146, "Subsequent bill must increment to 146");

    // 6. Verify reopening already migrated database does not re-migrate or fail
    drop(db);
    let reopened = Database::open(&db_path).expect("Reopening v4 database must succeed directly");
    let menu_reopened = reopened.get_menu().unwrap();
    assert!(menu_reopened.iter().any(|m| m.code == "999"));
}
