#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
#[derive(Eq, Debug, PartialEq, Hash, Copy, Clone)]
pub enum UomWeight {
    Pound,
    Kilo,
    Stone,
}


lazy_static! {
    static ref CONVERSION_MAP: HashMap<UomWeight, f64> = {
        let mut m = HashMap::new();
        m.insert(UomWeight::Pound, 2.2);
        m.insert(UomWeight::Kilo, 1.0);
        m.insert(UomWeight::Stone, 0.15);
        m
    };
}

pub trait ConvertWeight {
    fn convert_weight(self, uom: UomWeight) -> f64;
}

impl ConvertWeight for ItemWeight {
    fn convert_weight(self, weight: UomWeight) -> f64 {

        let base_in_kilos = &self.weight * CONVERSION_MAP.get(&UomWeight::Kilo).unwrap();
        base_in_kilos * CONVERSION_MAP.get(&weight).unwrap()

    }
}

#[derive(Debug, Copy, Clone)]
pub struct ItemWeight {
    weight: f64,
    uom: UomWeight,
}

#[derive(Debug)]
struct InventoryItem {
    item_number: u64,
    pub item_weight: ItemWeight,
}

impl InventoryItem {
    fn new(item_number: u64, item_weight: ItemWeight) -> Self {
        Self {
            item_number: item_number,
            item_weight: item_weight,
        }
    }
}

fn main() {
    let test_item_weight = ItemWeight {
        weight: 22.22,
        uom: UomWeight::Pound,
    };

    let an_item = &InventoryItem::new(333, test_item_weight);
    let converted_weight = an_item.item_weight.convert_weight(UomWeight::Stone);

    println!("The Item is: {:?}", &an_item);
    println!("The converted weight is: {:?}", converted_weight);
}
