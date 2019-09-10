#[derive(Debug)]
enum Uom_Weight { Kilos, Pounds, Stone }  //UOM -> Unit of Measure
const KILOS_TO_POUNDS:f32 = 2.20462;
const KILOS_TO_STONE:f32 = 0.157473;
// 2.20462 => Kilos to Pounds
// 0.157473 => Kilos to Stone

#[derive(Debug)]
struct InventoryItem {
    pub name: String,
    pub weight: f32,
    pub unit_type: Uom_Weight
} //...more code


fn main() {
    //1: create an inventory item
    let mut pounditem = InventoryItem{ name: String::from("pound item"), weight: 10.0, unit_type: Uom_Weight::Pounds };
    let mut stoneitem = InventoryItem{ name: String::from("stone item"), weight: 10.0, unit_type: Uom_Weight::Pounds };
    
    //3: output the converted value 
    // Use a trait
    pounditem.to_kilos();
    stoneitem.to_kilos();
    println!("kilos_from_pounditem => {:#?}", pounditem);
    println!("kilos_from_stoneitem => {:#?}", stoneitem);
    println!("-----------------------------------");
    println!("oops, I broke funtional programing.");
}

trait Convertor {
    fn to_kilos(&mut self);
}


impl Convertor for InventoryItem {
    fn to_kilos(&mut self){
        match self.unit_type{
            Uom_Weight::Pounds => { self.weight = self.weight * KILOS_TO_POUNDS},
            Uom_Weight::Stone => { self.weight = self.weight * KILOS_TO_STONE},
            Uom_Weight::Kilos =>{}
        }
        self.unit_type = Uom_Weight::Kilos;
    }
}

