#[derive(Debug)]
enum UomWeight { Kilos, Pounds, Stone }  //UOM -> Unit of Measure

// 2.20462 => Kilos to Pounds
// 0.157473 => Kilos to Stone
// 14 => Stone to Pounds

trait Convertor{
    fn to_pounds(&self) -> InventoryItem;
    fn to_stone(&self) -> InventoryItem;
    fn to_kilos(&self) -> InventoryItem;
}

#[derive(Debug)]
struct InventoryItem {
    value: f32,
    unit : UomWeight
} 

impl Convertor for InventoryItem{
    fn to_pounds(&self) -> InventoryItem{
        match self.unit {
            UomWeight::Kilos => InventoryItem{
                                    value : self.value * 2.20462,
                                    unit : UomWeight::Pounds
                                },
            UomWeight::Stone => InventoryItem{
                                    value : self.value * 14.0,
                                    unit : UomWeight::Pounds
                                },
            _ => InventoryItem {
                                    value : self.value,
                                    unit : UomWeight::Pounds
                                }
        }
        
    }
    fn to_stone(&self) -> InventoryItem{
        match self.unit {
            UomWeight::Kilos => InventoryItem{
                                    value : self.value *  0.157473,
                                    unit : UomWeight::Stone
                                },
            UomWeight::Pounds => InventoryItem{
                                    value : self.value / 14.0,
                                    unit : UomWeight::Stone
                                },
            _ => InventoryItem {
                                    value : self.value,
                                    unit : UomWeight::Stone
                                }
        }
    }
    fn to_kilos(&self) -> InventoryItem{
        match self.unit {
            UomWeight::Pounds => InventoryItem{
                                    value : self.value / 2.20462,
                                    unit : UomWeight::Kilos
                                },
            UomWeight::Stone => InventoryItem{
                                    value : self.value / 0.157473,
                                    unit : UomWeight::Kilos
                                },
            _ => InventoryItem {
                                    value : self.value,
                                    unit : UomWeight::Kilos
                                }
        }
    }
}

fn main() {

    //1: create an inventory item
    let item_kilos = InventoryItem { value: 1.0, unit: UomWeight::Kilos };
    let item_pounds = InventoryItem { value: 1.0, unit: UomWeight::Pounds };
    let item_stone = InventoryItem { value: 1.0, unit: UomWeight::Stone };

    //3: output the converted value 
    let result_kilos_to_pounds = item_kilos.to_pounds();
    let result_kilos_to_stone = item_kilos.to_stone();
    let result_kilos_to_kilos = item_kilos.to_kilos();
    println!("result_kilos_to_pounds => {:#?}", result_kilos_to_pounds);
    println!("result_kilos_to_stone => {:#?}", result_kilos_to_stone);
    println!("result_kilos_to_kilos => {:#?}", result_kilos_to_kilos);

    let result_pounds_to_pounds = item_pounds.to_pounds();
    let result_pounds_to_stone = item_pounds.to_stone();
    let result_pounds_to_kilos = item_pounds.to_kilos();
    println!("result_pounds_to_pounds => {:#?}", result_pounds_to_pounds);
    println!("result_pounds_to_stone => {:#?}", result_pounds_to_stone);
    println!("result_pounds_to_kilos => {:#?}", result_pounds_to_kilos);

    let result_stone_to_pounds = item_stone.to_pounds();
    let result_stone_to_stone = item_stone.to_stone();
    let result_stone_to_kilos = item_stone.to_kilos();
    println!("result_stone_to_pounds => {:#?}", result_stone_to_pounds);
    println!("result_stone_to_stone => {:#?}", result_stone_to_stone);
    println!("result_stone_to_kilos => {:#?}", result_stone_to_kilos);

    //Use a trait => Already impl Convertor (trait) for InventoryItem
}

