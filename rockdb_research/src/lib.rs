#[inline]
pub fn fibonacci(n: i32) -> i32 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

use rocksdb::DB;

#[inline]
pub fn rocksdb_write() {
    //NB: db is automatically closed at end of lifetime
    let db = DB::open_default("path/for/rocksdb/storage").unwrap();

    for i in 0..1200 {
        let key = format!("rock{}", i).to_string();
        let value = i.to_string();
        db.put(key, value);
    }

    return;
}

#[inline]
pub fn rocksdb_read() {
    //NB: db is automatically closed at end of lifetime
    let db = DB::open_default("path/for/rocksdb/storage").unwrap();

    for i in 0..1200 {
        let key = format!("rock{}", i).to_string();

        match db.get(key) {
            Ok(Some(value)) => return,
            Ok(None) => return,
            Err(e) => return,
        }
    }

    return;
}