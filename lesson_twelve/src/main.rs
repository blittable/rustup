
//1: create an inventory item
//2: output the converted value 
//3: Use a trait

// 1 kilogram = 2.20462262184878 pounds
// pounds = 2.2 * kilograms

pub const KILOGRAM_FACTOR: f64 = 1.0;
pub const KILOGRAM_TONNE_FACTOR: f64 = 14.0;
pub const KILOGRAM_POUND_FACTOR: f64 = 2.20462262184878;
pub const KILOGRAM_STONE_FACTOR: f64 = 0.157473;

enum UomWeight { Kilos, Pounds, Stone }
struct InventoryItem { weight: f64, uom: UomWeight }

pub trait MeasurementConverter {
    fn to_pounds(&self) -> f64; 
    fn to_kilos(&self) -> f64;
    fn to_stone(&self) -> f64;
}

impl MeasurementConverter for InventoryItem {
    fn to_pounds(&self) -> f64 {
        match self.uom {
            UomWeight::Pounds => self.weight, 
            UomWeight::Kilos => self.weight * KILOGRAM_POUND_FACTOR, 
            UomWeight::Stone => self.weight * KILOGRAM_TONNE_FACTOR
        }
    }   

    fn to_kilos(&self) -> f64 {
        match self.uom {
            UomWeight::Kilos => self.weight, 
            UomWeight::Pounds => self.weight / KILOGRAM_POUND_FACTOR, 
            UomWeight::Stone => self.weight / KILOGRAM_STONE_FACTOR
        }
    }

    fn to_stone(&self) -> f64 {
        match self.uom {
            UomWeight::Stone => self.weight, 
            UomWeight::Pounds => self.weight / KILOGRAM_TONNE_FACTOR, 
            UomWeight::Kilos => self.weight * KILOGRAM_STONE_FACTOR
        }
    }   
}

pub fn print_table_header() {    
    println!("\n{0: <30} | {1: <30} | {2: <30}","Pounds", "Kilos", "Stone");
}

pub fn display_convert<T>(mass: T) where T: MeasurementConverter {                    
    println!("{0: <30} | {1: <30} | {2: <30} ", mass.to_pounds(), mass.to_kilos(), mass.to_stone());    
}

fn main() {     
    print_table_header();
    display_convert(InventoryItem {weight: KILOGRAM_FACTOR, uom: UomWeight::Pounds}); 
    display_convert(InventoryItem {weight: KILOGRAM_FACTOR, uom: UomWeight::Kilos}); 
    display_convert(InventoryItem {weight: KILOGRAM_FACTOR, uom: UomWeight::Stone});    
}