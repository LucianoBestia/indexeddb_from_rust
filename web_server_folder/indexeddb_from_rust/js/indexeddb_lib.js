import * as idb from '/indexeddb_from_rust/idb/index.js';
/// does the browser have indexedDB
export function check_browser_capability() {
    if (!window.indexedDB) {
        console.log("NO, IndexedDB is not available.");
    }
    else {
        console.log("Yes, Indexeddb is available");
    }
}
/// open db with upgrade code, returns a promise
export async function js_open_db() {
    console.log("js_open_db");
    let db1 = await idb.openDB('db1', 1, {
        upgrade(db) {
            console.log("upgrade(db)");
            db.createObjectStore('currency');
        },
    });
    return db1;
}
/// add key-value in a store
export async function add_key_value(db1, store, key, value) {
    console.log("add");
    db1.add(store, value, key);
}
/// put key-value in a store (upsert)
export async function put_key_value(db1, store, key, value) {
    console.log("put");
    db1.put(store, value, key);
}
/// get key-value in a store 
export async function get_key_value(db1, store, key) {
    console.log("get");
    db1.get(store, key);
    const value = await db1.get(store, key);
    return value;
}
//# sourceMappingURL=indexeddb_lib.js.map