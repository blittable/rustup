pub mod water;
pub mod milk;

pub trait Property {
    fn state(&self) -> Option<String>;
}