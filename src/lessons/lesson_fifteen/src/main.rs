#[allow(dead_code)]

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
    let house_a = House::new().setup_house(5, 2);
    let house_e = House::new().setup_house(3, 2);
    let house_d = House::new().setup_house(3, 2);

    //Create the streets
    let mut street_0 = Street { houses: Vec::new() };
    let mut street_1 = Street { houses: Vec::new() };

    street_0.houses.push(house_a);
    street_0.houses.push(house_e);

    //house_d is on both streets
    street_1.houses.push(house_d);
    street_0.houses.push(house_d);

    println!("House A\t: Size {:?}", house_a.get_house_size());
    println!("\t: {:?}", house_a.details());
    println!("House D\t: Size {:?}", house_d.get_house_size());
    println!("\t: {:?}", house_d.details());
    println!("House E\t: Size {:?}", house_e.get_house_size());
    println!("\t: {:?}", house_e.details());
}

#[derive(Copy, Clone)]
struct Room {
    name: *mut String,
    room_size: i32,
}

#[derive(Copy, Clone)]
struct House {
    rooms: *mut Vec<Room>,
}

struct Street {
    houses: Vec<House>,
}

impl Room {
    pub fn details(mut self) -> String {
        let name = unsafe { Box::from_raw(self.name) };
        let details = format!("{}: {}", name, self.room_size);
        self.name = Box::into_raw(name);
        details
    }
}

impl House {
    pub fn new() -> House {
        House { rooms: Box::into_raw(Box::new(Vec::new())) }
    }

    pub fn get_house_size(mut self) -> i32 {
        let rooms = unsafe { Box::from_raw(self.rooms) };
        let size = rooms.iter().map(|x| x.room_size).sum();
        self.rooms = Box::into_raw(rooms);
        size
    }

    pub fn setup_house(mut self, bedrooms: i8, living_rooms: i8) -> House {
        for i in 0..bedrooms {
            let bedroom: Room = Room {
                name: Box::into_raw(Box::new(format!("bedroom number {}", i))),
                room_size: 20,
            };
            let mut rooms = unsafe { Box::from_raw(self.rooms) };
            rooms.push(bedroom);
            self.rooms = Box::into_raw(rooms);
        }

        for i in 0..living_rooms {
            let living_room: Room = Room {
                name: Box::into_raw(Box::new(format!("living room number {}", i))),
                room_size: 34,
            };
            let mut rooms = unsafe { Box::from_raw(self.rooms) };
            rooms.push(living_room);
            self.rooms = Box::into_raw(rooms);
        }
        self
    }

    pub fn details(&self) -> Vec<String> {
        let rooms = unsafe { Box::from_raw(self.rooms) };
        rooms.iter().map(|r| r.details()).collect()
    }
}