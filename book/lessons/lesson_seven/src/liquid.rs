pub mod is;

pub trait Property {
    fn state(&self) -> String;
}