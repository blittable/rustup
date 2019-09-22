enum RoomType {
    Bedroom,
    Bathroom,
    Kitchen,
    Dining,
    Recreation,
    LivingRoom,
}

impl Default for RoomType {
    fn default() -> Self {
        RoomType::Bedroom
    }
}

struct House {
    size_meters: i16,
    rooms: Vec<Room>,
}

impl House {
    pub fn new(size: i16) -> Self {
        Self {
        size_meters: size,
        rooms: Vec::<Room>::new(),
        }
    }

    pub fn add_room(&mut self, room: Room) {
        self.rooms.push(room)
    }

    pub fn get_room_count(&self) -> usize {
        self.rooms.len()
    }
}

struct Room {
    size_meters: i16,
    usage: RoomType,
}

fn main() {
    let mut basic_house: House = House::new(288);

    let room_a: Room = Room { usage: RoomType::Bathroom, size_meters: 10 }; 
    let room_b: Room = Room { usage: RoomType::Bedroom, size_meters: 10 }; 

    basic_house.add_room(room_a);
    basic_house.add_room(room_b);

    println!("Number of Rooms: {:?}", basic_house.get_room_count())
}
