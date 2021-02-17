// currdb_mod

// Specific code for work with the `currdb` database

use crate::idb_mod as idb;
use crate::web_sys_mod as w;
use strum::AsStaticRef;
use strum_macros::*;
use unwrap::unwrap;
use wasm_bindgen::prelude::*;

// Enums for databases and object stores to avoid mistyping the names
// in Rust I can use enum, but for javascript I will convert it to &'static str with .as_static()
#[derive(AsStaticStr)]
pub enum Databases {
    Currdb,
}

#[derive(AsStaticStr)]
pub enum ObjectStores {
    Currency,
    Config,
}

/// init_upgrade_currdb will create the database and call upgrade_currdb()
pub async fn init_upgrade_currdb() {
    // pointer for rust closure on the Heap
    let rust_closure_for_upgrade = Closure::wrap(Box::new(
        move |db: JsValue, old_version: JsValue, new_version: JsValue, transaction: JsValue| {
            upgrade_currdb(db, old_version, new_version, transaction);
        },
    )
        as Box<dyn Fn(JsValue, JsValue, JsValue, JsValue)>);
    idb::Database::init_upgrade_db(&Databases::Currdb.as_static(), 2, &rust_closure_for_upgrade)
        .await;
}

/// pass this function to javascript as closure
pub fn upgrade_currdb(
    db: JsValue,
    old_version: JsValue,
    new_version: JsValue,
    transaction: JsValue,
) {
    let db = idb::Database::from(db);
    let tx = idb::Transaction::from(transaction);
    let old_version = unwrap!(old_version.as_f64()) as i32;
    let new_version = unwrap!(new_version.as_f64()) as i32;
    w::debug_write(&format!(
        "upgrade_currdb from v{} to v{}",
        old_version, new_version
    ));

    if old_version < 1 {
        upgrade_from_v00_to_v01(&db);
    }
    if old_version < 2 {
        upgrade_from_v01_to_v02(&db, &tx);
    }
}

fn upgrade_from_v00_to_v01(db: &idb::Database) {
    w::debug_write("upgrade_from_v00_to_v01");
    db.create_object_store(&ObjectStores::Currency.as_static());
}

fn upgrade_from_v01_to_v02(db: &idb::Database, tx: &idb::Transaction) {
    w::debug_write("upgrade_from_v01_to_v02");
    db.create_object_store(&ObjectStores::Config.as_static());
    let cfg = tx.get_object_store_versionchange(&ObjectStores::Config.as_static());
    // this is a special put inside a transaction, that is inside version upgrade
    cfg.put("base_currency", "EUR");
    cfg.put("quote_currency", "USD");
    cfg.put("rate", "1.21");
    cfg.put("date_fetch", "none");
}