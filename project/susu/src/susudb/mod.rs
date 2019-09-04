use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
// use std::io::SeekFrom;
use std::path::Path;

#[allow(non_snake_case)]
pub mod SusuExceptions;
pub mod traits;
use crate::susudb::traits::*;
use crate::susudb::SusuExceptions::SusuDbException;

const KEY_VALUE_SEPARATOR: &str = "=su=";
const ITEM_SEPARATOR: &str = "\n";

// -- SusuData
#[derive(Debug)]
pub struct SusuData {
    pub key: String,
    pub value: Option<String>,
}

impl SusuDataTraits for SusuData {
    fn new() -> Self {
        Self {
            key: String::new(),
            value: None,
        }
    }

    fn new_data(key: &str, value: &str) -> Self {
        let mut check_value: Option<String> = None;
        if !value.is_empty() {
            check_value = Some(value.to_string());
        }

        Self {
            key: String::from(key),
            value: check_value,
        }
    }

    fn to_data_format(&self) -> Result<String, SusuDbException> {
        if self.key.is_empty() {
            return Err(SusuDbException::new("Key is null"));
        }

        let result = format!(
            "{}{}{}",
            self.key,
            KEY_VALUE_SEPARATOR,
            self.value.as_ref().unwrap()
        );
        Ok(result)
    }
}

// -- SusuDatabase
pub struct SusuDatabase {
    pub db_name: String,
}

impl SusuDatabase {
    fn do_add(&self, item: SusuData) -> bool {
        let data = item.to_data_format();
        match data {
            Ok(mut key_value) => {
                // Add new line
                key_value.push_str(ITEM_SEPARATOR);

                // Open file with append mode
                let mut file = OpenOptions::new()
                    .append(true)
                    .open(&self.db_name)
                    .expect("Couldn't open file");

                // Write string to file
                file.write_all(key_value.as_bytes())
                    .expect("Couldn't write file");

                return true;
            }
            Err(err) => panic!("Error: {:?}", err.description()),
        }
    }

    fn do_update(&self, exist_item: SusuData, update_item: SusuData) -> bool {
        if exist_item.value == update_item.value {
            // Same item no need to update!
            return false;
        }

        // Open file with read mode
        let mut file_reader = File::open(&self.db_name).expect("Couldn't open file");
        // Read string content in file
        let mut content_strings = String::new();
        file_reader
            .read_to_string(&mut content_strings)
            .expect("Couldn't read file");

        let exist_string: String = exist_item.to_data_format().unwrap();
        let update_string: String = update_item.to_data_format().unwrap();
        content_strings = content_strings.replace(&exist_string, &update_string);

        // Open file with write mode
        let mut file_writer = File::create(&self.db_name).expect("Couldn't open file");
        file_writer
            .write_all(content_strings.as_bytes())
            .expect("Couldn't write file");

        return true;
    }

    fn do_get(&self, key: &str) -> Option<SusuData> {
        // Open file
        let mut file_reader = File::open(&self.db_name).expect("Couldn't open file");
        // file_reader.seek(SeekFrom::Start(5)).expect("Couldn't read file");

        // Read string content in file
        let mut content_strings = String::new();
        file_reader
            .read_to_string(&mut content_strings)
            .expect("Couldn't read file");

        let fn_susu_map = |s: String| -> SusuData {
            let kv: Vec<&str> = s.split(KEY_VALUE_SEPARATOR).collect();
            let mut susu_data = SusuData::new();
            if !kv.is_empty() && kv.len() > 1 {
                susu_data.key = kv[0].to_string();
                susu_data.value = Some(kv[1].to_string());
            }

            return susu_data;
        };

        let susu_result = content_strings
            //.lines()
            .split(ITEM_SEPARATOR)
            .filter(|s| s.contains(&key))
            .map(|s| s.to_string())
            .collect::<HashSet<_>>()
            .into_iter()
            .map(|s| fn_susu_map(s))
            .filter(|s| !s.key.is_empty() && s.key == key)
            .next();

        return susu_result;
    }
}

impl SusuDatabaseTraits for SusuDatabase {
    fn new() -> Self {
        Self {
            db_name: String::new(),
        }
    }

    fn config(&mut self, db_name: &'static str) {
        let mut db: String = db_name.to_string();
        if !db_name.to_string().ends_with(".txt") {
            db = format!("{}.txt", db_name);
        }

        let path = Path::new(&db);
        let display = path.display();
        if !path.exists() {
            match File::create(&path) {
                Err(err) => panic!("Couldn't create {} file, {}", display, err.description()),
                Ok(file) => file,
            };
        }

        self.db_name = db.to_owned();
    }

    fn add(&self, item: SusuData) -> bool {
        let exist_data = self.do_get(&item.key);

        if exist_data.is_none() {
            println!("> Adding item: {:?}", item);

            // Add new data
            return self.do_add(item);
        } else {
            println!("> Updating existing item: {:?} to {:?}", &exist_data, &item);

            // Update data
            return self.do_update(exist_data.unwrap(), item);
        }
    }

    fn get(&self, key: &str) -> Option<String> {
        println!("> Getting item by key: {:?}", key);

        match self.do_get(key) {
            Some(s) => match s.value {
                Some(r) => Some(r),
                None => None,
            },
            None => None,
        }
    }
}
