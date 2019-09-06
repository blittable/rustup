use crate::SusuData;
use crate::susuexceptions::SusuDbException;

pub trait SusuDataTrait {
    fn new() -> Self;
    fn new_data(key: &str, value: &str) -> Self;
    fn to_data_format(&self) -> Result<String, SusuDbException>;
}

pub trait SusuDatabaseTrait {
    fn new() -> Self;
    fn config(&mut self, db_name: &'static str);
    fn get(&self, key: &str) -> Option<String>;
    fn add(&self, item: SusuData) -> bool;
}
