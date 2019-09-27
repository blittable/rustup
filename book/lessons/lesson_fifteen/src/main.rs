#[allow(dead_code)]
use std::cell::{ Ref, RefCell};
///
/// We have a hypothetical neighborhood that looks like this:
///
///                   HOUSE_A   HOUSE_B    HOUSE_C     HOUS
///                   ---------------------------------   E
///                            SOI 13                  |  _
///                   ---------------------------|     |  D
///                                              |     |  
///                                              |  S  |  
///                                              |  O  |
///                                              |  I  |
///                                              |     |  H
///                                              |  1  |  O
///                                              |  2  |  U
///                                              |     |  S
///                                              |     |  E
///                                              |     |  _
///                                              |     |  E
///
///
/// Noteably, HOUSE_D, is on both Soi 12 and 13
///
/// If street contains N houses, we have a problem with this simple implementation.
///
/// We have 3 first solutions to consider:
/// 1: Box<T>: Heap allocate and wrap with a stack reference
/// 2: Rc<T>: Wrap with a counted, shareable immutable reference, compile-time checked, for single threaded scenarios
/// 3: RcCell<T> Wrap with a counted, shareable mutable reference, runtime checked

fn main() {
    //setup a house
    let house_a = House::new().setup_house(1, 2);
    let house_e = House::new().setup_house(3, 2);
    let house_d = House::new().setup_house(2, 1);

    //Create the streets
    let mut street_0 = Street { houses: Vec::new() };
    let mut street_1 = Street { houses: Vec::new() };

    street_1.houses.push(house_a.borrow());
    street_0.houses.push(house_e.borrow());

    //house_d is on both streets
    street_1.houses.push(house_d.borrow());
    street_0.houses.push(house_d.borrow());

    println!("Street 0:\t\t");
    for h in street_0.houses {
        println!("\tHouse Size\t:{:?}", &h.get_house_size());
        println!("\tHouse Plan\t:{:?}", &h.details());
    }
    println!("Street 1:\t\t");
    for h in street_1.houses {
        println!("\tHouse Size\t:{:?}", &h.get_house_size());
        println!("\tHouse Plan\t:{:?}", &h.details());
    }
}

#[derive(Debug)]
struct Room {
    name: String,
    room_size: i32,
}

#[derive(Debug)]
struct House {
    rooms: Vec<Room>,
}

#[derive(Debug)]
struct Street<'a> {
    houses: Vec<Ref<'a,House>>,
}

impl House {
    pub fn new() -> House {
        House { rooms: Vec::new() }
    }

    pub fn get_house_size(&self) -> i32 {
        self.rooms.iter().map(|x| x.room_size).sum()
    }

    pub fn setup_house(mut self, bedrooms: i8, living_rooms: i8) -> RefCell<House> {
        for i in 0..bedrooms {
            let bedroom: Room = Room {
                name: format!("bedroom number {}", i),
                room_size: 20,
            };
            self.rooms.push(bedroom);
        }

        for i in 0..living_rooms {
            let living_room: Room = Room {
                name: format!("living room number {}", i),
                room_size: 34,
            };
            self.rooms.push(living_room);
        }
        RefCell::new(self)
    }

    pub fn details(&self) -> Vec<String> {
        self.rooms.iter().map(|r| format!("{}: {}", &r.name, &r.room_size)).collect()
    }
}