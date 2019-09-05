pub mod susuexceptions;
pub mod traits;

// use std::collections::HashSet;
use crate::susuexceptions::SusuDbException;
use crate::traits::{SusuDataTrait, SusuDatabaseTrait};

use std::error::Error;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
// use std::io::SeekFrom;
use crc32fast::Hasher;
use std::path::Path;

const KEY_VALUE_SEPARATOR: &str = "=su=";
const ITEM_SEPARATOR: &str = "\n";

// -- SusuData
#[derive(Debug)]
pub struct SusuData {
    pub key: String,
    pub value: Option<String>,
}

impl SusuDataTrait for SusuData {
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
    fn get_filename(&self, key: &str) -> String {
        let mut hasher = Hasher::new();
        hasher.update(key.as_bytes());
        return hasher.finalize().to_string();
    }

    fn do_add(&self, item: SusuData) -> bool {
        let data = item.to_data_format();
        let filepath = format!("{}/{}", &self.db_name, &self.get_filename(&item.key));
        match data {
            Ok(mut key_value) => {
                // Add new line
                key_value.push_str(ITEM_SEPARATOR);

                // Open file with append mode
                let mut file_writer = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(filepath)
                    .expect("Couldn't open file");

                // Write string to file
                file_writer
                    .write_all(key_value.as_bytes())
                    .expect("Couldn't write file");

                return true;
            }
            Err(err) => panic!("Error: {:?}", err.description()),
        }
    }

    fn do_update(
        &self,
        exist_item: SusuData,
        update_item: SusuData,
        current_strings: Option<String>,
    ) -> bool {
        if exist_item.value == update_item.value {
            // Same item no need to update!
            println!(
                "duplicate (key: {:?}, value: {:?})",
                exist_item.key,
                exist_item.value.unwrap()
            );
            return false;
        }

        let filepath = format!("{}/{}", &self.db_name, &self.get_filename(&exist_item.key));
        let mut content_strings = String::new();
        if current_strings.is_none() {
            // Open file with read mode
            let mut file_reader = File::open(&filepath).expect("Couldn't open file");
            // Read string content in file
            file_reader
                .read_to_string(&mut content_strings)
                .expect("Couldn't read file");
        } else {
            content_strings = current_strings.unwrap();
        }

        if content_strings.is_empty() {
            return false;
        }

        let exist_string: String = exist_item.to_data_format().unwrap();
        let update_string: String = update_item.to_data_format().unwrap();
        content_strings = content_strings.replace(&exist_string, &update_string);

        // Open file with write mode
        let mut file_writer = File::create(&filepath).expect("Couldn't open file");
        file_writer
            .write_all(content_strings.as_bytes())
            .expect("Couldn't write file");

        return true;
    }

    fn do_get(
        &self,
        key: &str,
        is_get_current_strings: bool,
    ) -> (Option<SusuData>, Option<String>) {
        // Open file
        let filepath = format!("{}/{}", &self.db_name, &self.get_filename(key));
        let path = Path::new(&filepath);
        if !path.exists() {
            return (None, None);
        }

        let mut file_reader = File::open(&filepath).expect("Couldn't open file");

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
            .split(ITEM_SEPARATOR)
            .filter(|s| s.starts_with(&key))
            .map(|s| s.to_string())
            .map(|s| fn_susu_map(s))
            .filter(|s| !s.key.is_empty() && s.key == key)
            .next();

        if is_get_current_strings {
            return (susu_result, Some(content_strings));
        } else {
            return (susu_result, None);
        }
    }
}

impl SusuDatabaseTrait for SusuDatabase {
    fn new() -> Self {
        Self {
            db_name: String::new(),
        }
    }

    fn config(&mut self, db_name: &'static str) {
        let db: String = db_name.to_string();

        let path = Path::new(&db);
        let display = path.display();
        if !path.exists() {
            match fs::create_dir(&path) {
                Err(err) => panic!("Couldn't create {}, {}", display, err.description()),
                Ok(()) => {}
            };
        }

        self.db_name = db.to_owned();
    }

    fn add(&self, item: SusuData) -> bool {
        let (exist_data, current_strings) = self.do_get(&item.key, true);

        if exist_data.is_none() {
            #[cfg(debug_assertions)]
            println!("> Adding item: {:?}", item);

            // Add new data
            return self.do_add(item);
        } else {
            #[cfg(debug_assertions)]
            println!("> Updating existing item: {:?} to {:?}", &exist_data, &item);

            // Update data
            return self.do_update(exist_data.unwrap(), item, current_strings);
        }
    }

    fn get(&self, key: &str) -> Option<String> {
        #[cfg(debug_assertions)]
        println!("> Getting item by key: {:?}", key);

        let (result, _) = self.do_get(key, false);
        match result {
            Some(s) => s.value,
            None => None,
        }
    }
}
