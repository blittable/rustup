extern crate susudb;
use crate::susudb::traits::{SusuDataTrait, SusuDatabaseTrait};

fn main() {
    let mut database = susudb::SusuDatabase::new();
    database.config("susu_db");

    // Test add data
    for key_temp in 0..1200 {
        let new_data =
            susudb::SusuData::new_data(&format!("key#{}", key_temp), &format!("rust_{}", key_temp));
        database.add(new_data);
    }

    // Test update data
    for key_temp in 0..1200 {
        let new_data = susudb::SusuData::new_data(
            &format!("key#{}", key_temp),
            &format!("rust_updated_v1_{}", key_temp),
        );
        database.add(new_data);
    }

    // Test get data
    for key_temp in 0..1200 {
        let value = database.get(&format!("key#{}", key_temp));
        match value {
            Some(val) => println!("Found value: {:?}\n", val),
            None => println!("Not found any data\n"),
        }
    }

    // // Test add invalid data
    // let new_data = susudb::SusuData::new_data(&"", &"rust_empty");
    // database.add(new_data);
}
