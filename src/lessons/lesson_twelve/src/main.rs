enum UomWeight { 
    Kilos,
    Pounds,
    Stone
}  //UOM -> Unit of Measure

impl UomWeight {
    fn value(&self) -> f32 {
        match *self {
            UomWeight::Kilos => 1.0,
            UomWeight::Pounds => 2.20462,
            UomWeight::Stone => 0.157473,
        }
    }
    fn unit(&self) -> String {
        match *self {
            UomWeight::Kilos => String::from("Kilogram"),
            UomWeight::Pounds => String::from("Pound"),
            UomWeight::Stone => String::from("Stone"),
        }
    }
}


// 2.20462 => Kilos to Pounds
// 0.157473 => Kilos to Stone

struct InventoryItem {
    mass: f32,
    pub convertor: Box<dyn Weightable>
} //...more code

trait Weightable {
    fn weight(&self) -> f32;
}

impl Weightable for InventoryItem {
    fn weight(&self) -> f32 {
        self.mass * self.convertor.weight()
    }
}

impl Weightable for UomWeight {
    fn weight(&self) -> f32 {
        self.value()
    }
}

fn main() {
    let item1 = InventoryItem {
        mass: 10.0,
        convertor: Box::new(UomWeight::Kilos),
    };

    println!(
        "Weight of Mass {} in {} = {}",
        item1.mass,
        UomWeight::Kilos.unit(),
        item1.weight()
    );

    let item2 = InventoryItem {
        mass: 10.0,
        convertor: Box::new(UomWeight::Pounds),
    };

    println!(
        "Weight of Mass {} in {} = {}",
        item2.mass,
        UomWeight::Pounds.unit(),
        item2.weight()
    );

    let item3 = InventoryItem {
        mass: 10.0,
        convertor: Box::new(UomWeight::Stone),
    };

    println!(
        "Weight of Mass {} in {} = {}",
        item3.mass,
        UomWeight::Stone.unit(),
        item3.weight()
    );
    //1: create an inventory item
    //3: output the converted value 
    // Use a trait
}