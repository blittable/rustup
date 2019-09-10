// etc3tera

use std::fmt;
use std::convert::From;

enum UomWeight { Kilos, Pounds, Stone }  //UOM -> Unit of Measure

// 2.20462 => Kilos to Pounds
// 0.157473 => Kilos to Stone

struct InventoryItem {
    item: Box<dyn KiloConvertable>,
    current_type: UomWeight
}
impl InventoryItem {
    fn convert_to(&mut self, target_type: UomWeight) {
        let tmp: Box<dyn KiloConvertable> = Box::new(StoneItem::from(&self.item));
        self.current_type = target_type;
        self.item = tmp;
    }
    fn new(value: f32, desired_type: UomWeight) -> Self {
        let tmp: Box<dyn KiloConvertable> = 
            match desired_type {
                UomWeight::Kilos => Box::new(KiloItem { value: value }),
                UomWeight::Pounds => Box::new(PoundItem{ value: value }),
                UomWeight::Stone => Box::new(StoneItem { value: value })
            };
        
        return InventoryItem { item: tmp, current_type: desired_type };
    }
}
impl fmt::Display for InventoryItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.current_type {
            UomWeight::Kilos => write!(f, "{}", KiloItem::from(&self.item)),
            UomWeight::Pounds => write!(f, "{}", PoundItem::from(&self.item)),
            UomWeight::Stone => write!(f, "{}", StoneItem::from(&self.item))
        }
    }
}

trait KiloConvertable {
    fn to_kilo(&self) -> f32;
}

struct StoneItem { value: f32 }
impl KiloConvertable for StoneItem {
    fn to_kilo(&self) -> f32 {
        self.value / 0.157473
    }
}
impl From<&Box<dyn KiloConvertable>> for StoneItem {
    fn from(obj: &Box<dyn KiloConvertable>) -> Self {
        StoneItem { value: obj.to_kilo() * 0.157473 }
    }
}
impl fmt::Display for StoneItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} Stones", self.value)
    }
}

struct KiloItem { value: f32 }
impl KiloConvertable for KiloItem {
    fn to_kilo(&self) -> f32 {
        self.value 
    }
}
impl From<&Box<dyn KiloConvertable>> for KiloItem {
    fn from(obj: &Box<dyn KiloConvertable>) -> Self {
        KiloItem { value: obj.to_kilo() }
    }
}
impl fmt::Display for KiloItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} Kilos", self.value)
    }
}

struct PoundItem { value: f32 }
impl KiloConvertable for PoundItem {
    fn to_kilo(&self) -> f32 {
        self.value / 2.20462
    }
}
impl From<&Box<dyn KiloConvertable>> for PoundItem {
    fn from(obj: &Box<dyn KiloConvertable>) -> Self {
        PoundItem { value: obj.to_kilo() * 2.20462 }
    }
}
impl fmt::Display for PoundItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} Pounds", self.value)
    }
}

// fn main() {
//     // Create and print in kilo unit
//     let x = PoundItem { value: 4f32 };
//     println!("{}", x.to_kilo());

//     // Convert from Self Unit
//     let y = PoundItem::from(PoundItem { value: 4f32 });
//     println!("{}", y);

//     // Convert from Other Unit
//     let z: Box<dyn KiloConvertable> = Box::new(StoneItem { value: 8f32 });
//     let z2 = PoundItem::from(&z);
//     println!("{}", z2);
//     println!("{}", StoneItem::from(&z));

//     println!();
//     println!("Use Inventory Item Structure");

//     let mut item = InventoryItem::new(8f32, UomWeight::Stone);
//     println!("Before convert: {}", item);
//     item.convert_to(UomWeight::Pounds);
//     println!("To Pounds: {}", item);
//     item.convert_to(UomWeight::Kilos);
//     println!("To Kilos: {}", item);
// }


use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            eprintln!("We need a command line argument!");
            return;
        }
        2 => {
            let cmd = &args[1];
            let i: i32 = match cmd.parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("error in parsing the command line argument at position 1");
                    return;
                }
            };

            validate_age(i)
        }
        _ => {
            eprintln!("Error in parameters");
        }
    };
}
