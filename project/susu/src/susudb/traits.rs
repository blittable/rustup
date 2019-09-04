use crate::susudb::SusuData;
use crate::susudb::SusuExceptions::SusuDbException;

pub trait SusuDataTraits {
    fn new() -> Self;
    fn new_data(key: &str, value: &str) -> Self;
    fn to_data_format(&self) -> Result<String, SusuDbException>;
}

pub trait SusuDatabaseTraits {
    fn new() -> Self;
    fn config(&mut self, db_name: &'static str);
    fn get(&self, key: &str) -> Option<String>;
    fn add(&self, item: SusuData) -> bool;
}
