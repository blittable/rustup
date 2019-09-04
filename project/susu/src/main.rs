mod susudb;

use crate::susudb::traits::SusuDataTraits;
use crate::susudb::traits::SusuDatabaseTraits;
use crate::susudb::SusuData;
use crate::susudb::SusuDatabase;

fn main() {
    println!("*** Hello, susu db! ***");

    let mut database = SusuDatabase::new();
    database.config("susu_db");

    // Test add data
    for key_temp in 0..100 {
        let new_data =
            SusuData::new_data(&format!("key#{}", key_temp), &format!("rust_{}", key_temp));
        database.add(new_data);
    }

    // Test update data
    for key_temp in 0..100 {
        let new_data = SusuData::new_data(
            &format!("key#{}", key_temp),
            &format!("rust_updated_v1_{}", key_temp),
        );
        database.add(new_data);
    }

    // Test get data
    for key_temp in 0..102 {
        let value = database.get(&format!("key#{}", key_temp));
        match value {
            Some(val) => println!("Found value: {:?}\n", val),
            None => println!("Not found any data\n"),
        }
    }

    // // Test add invalid data
    // let new_data = SusuData::new_data(&"", &"rust_empty");
    // database.add(new_data);
}
