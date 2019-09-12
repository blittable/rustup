#[allow(dead_code)]
use std::rc::Rc;

struct House {
    id: u32,
    owner: String,
    street: Vec<Rc<Street>>,
}

struct Street {
    number: u16,
    houses: Vec<House>, // <- Add a lifetime annotation
}

impl Street {
    pub fn new(new_number: u16) -> Street {
        Street {
            number: new_number,
            houses: Vec::new(),
        }
    }
}

impl House {
    pub fn new(new_id: u32, new_owner: String, associated_street: Vec<Rc<Street>>) -> House {
        House {
            id: new_id,
            owner: new_owner,
            street: associated_street,
        }
    }
}

pub fn main() {
    //Create the streets
    let mut street_0 = Rc::new(Street::new(10));
    let mut street_1 = Rc::new(Street::new(13));

    let mut single_street = Vec::new();
    single_street.push(Rc::clone(&street_0));

    let mut street_pair = Vec::new();
    street_pair.push(Rc::clone(&street_0));
    street_pair.push(Rc::clone(&street_1));

    //setup three houses
    let house_a = House::new(0, "Smith".to_string(), single_street);
    let house_d = House::new(2, "Douglas".to_string(), street_pair);
}