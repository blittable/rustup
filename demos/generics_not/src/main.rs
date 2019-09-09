#[derive(Debug, Clone)]
enum FoodGroup { Meat, Vegetables }

#[derive(Debug, Clone)]
struct FoodItem {
    food_type: FoodGroup,
    description: String
}

struct Refrigerator{
    contents: Vec<FoodItem>,
}

trait FoodStorage {
    fn add_item(self, food_item: &FoodItem);
}

impl Refrigerator {
    fn new() -> Refrigerator {
        Refrigerator {
            contents: Vec::new()
        }
    }
}    

impl FoodStorage for Refrigerator {
    fn add_item(mut self, food_item: &FoodItem) {
        self.contents.push(*food_item)
    }
}

fn main() {

    let frig: &mut Refrigerator = &mut Refrigerator::new();
    let hamburger: & FoodItem = &FoodItem { description: "Burger".to_string(), food_type: FoodGroup::Meat };
    //frig.add_item(hamburger);

    for i in frig.contents.iter() {
        println!("What's in the refrigerator? : {:?}", i);
    }
}

