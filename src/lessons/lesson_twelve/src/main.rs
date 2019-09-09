fn main() {
    println!("Hello, world!");

    //let 
}

enum Uom_Weight { Kilos, Pounds, Stones }  //UOM -> Unit of Measure

trait ConvertableWeight {
    fn convert<T>(from: T, to: T) -> f32;
} 

struct InventoryItem<T>
where T: ConvertableWeight {}

impl<T> InventoryItem<T> {
    fn convert<T>(from: T, to: T) -> f32 {
        Uom::Kilos * Uom::Pounds
    } 
}

fn amazing_code<T> (passport: T) {

} 