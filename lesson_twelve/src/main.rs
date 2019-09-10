enum Uom_Weight { Kilos, Pounds, Stone }  //UOM -> Unit of Measure

// 2.20462 => Kilos to Pounds
// 0.157473 => Kilos to Stone

trait ConvertToPounds {
    fn calculate_kilos_to_pounds(&self, amount: f32, unit: f32) -> f32;
}

trait ConvertToStone {
    fn calculate_kilos_to_stone(&self, amount: f32, unit: f32) -> f32;
}

struct InventoryPounds {
    pub weight_calculator: Box<dyn ConvertToPounds>
}

struct InventoryStone {
    pub weight_calculator: Box<dyn ConvertToStone>
}

struct InventoryItem { } 

impl ConvertToPounds for InventoryItem {
    fn calculate_kilos_to_pounds(&self, amount: f32, unit: f32) -> f32 {
        amount * unit
    }
}

impl ConvertToStone for InventoryItem {
    fn calculate_kilos_to_stone(&self, amount: f32, unit: f32) -> f32 {
        amount * unit
    }
}

fn main() {

    let item_a = InventoryPounds {
        weight_calculator: Box::new(InventoryItem {})
    };

    let item_b = InventoryStone {
        weight_calculator: Box::new(InventoryItem {})
    };

    println!("item A weight: {} km --> convert to {} pounds", 50.0, item_a.weight_calculator.calculate_kilos_to_pounds(50.0, 2.20462));

    println!("item B weight: {} km --> convert to {} stone", 50.0, item_b.weight_calculator.calculate_kilos_to_stone(50.0, 0.157473));

}
