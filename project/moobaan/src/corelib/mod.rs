#[allow(dead_code)]
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::rc::Rc;

///The owner of 1 or more Development Companies
#[derive(Serialize, Deserialize)]
struct HoldingCompany {
    company_id: u32,
    name: String,
    subsidiaries: Vec<DevelopmentCompany>,
}

impl HoldingCompany {
    pub fn new(new_company_id: u32, new_name: String) -> HoldingCompany {
        HoldingCompany {
            company_id: new_company_id,
            name: new_name,   
            subsidiaries: Vec::new(),
        }
    }
}

///A company that builds (and manages) Moobaans
#[derive(Serialize, Deserialize)]
struct DevelopmentCompany {
    company_id: u32,
    name: String,
    moo_baans: Vec<MooBaan>,
}

impl DevelopmentCompany {
    pub fn new(new_id: u32, new_name: String) -> DevelopmentCompany {
        DevelopmentCompany {
            company_id: new_id,
            name: new_name,   
            moo_baans: Vec::new(),
        }
    }
}

///A moo baan (housing estate, or village)
#[derive(Serialize, Deserialize)]
struct MooBaan {
    id: u32,
    name: String,
    houses: Vec<House>,
    streets: Vec<Street>,
} 

impl MooBaan {
    pub fn new(new_id: u32, new_name: String) -> MooBaan {
        MooBaan {
            id: new_id,
            name: new_name,   
            houses: Vec::new(),
            streets: Vec::new(),
        }
    }
}

///A room in a house
#[derive(Serialize, Deserialize)]
struct Room {
    name: String,
    room_size: u32,
}

//A place where people live having N rooms
#[derive(Serialize, Deserialize)]
struct House {
    id: u32,
    rooms: Vec<Room>,
}

//A street with N houses
#[derive(Serialize, Deserialize)]
struct Street {
    number: u16,
    houses: Vec<House>, // <- Add a lifetime annotation
}

impl Street{
    pub fn new(new_number: u16) -> Street {
        Street {
            number: new_number,
            houses: Vec::new(),
        }
    }
}

impl House {
    pub fn new(new_id: u32) -> House {
        House {
            id: new_id,
            rooms: Vec::new(),
        }
    }

    pub fn get_house_size(&self) -> u32 {
        self.rooms.iter().map(|x| x.room_size).sum()
    }

    pub fn setup_house(mut self, bedrooms: i8, living_rooms: i8) -> House {
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
        self
    }
}

pub fn build_it() -> Result<()> {

    //setup a house
    let house_a = House::new(0).setup_house(5, 2);
    let house_e = House::new(1).setup_house(3, 2);
    let house_d = House::new(2).setup_house(3, 2);

    //Create the streets
    let mut street_0 = Street::new(10); 
    let mut street_1 = Street::new(13); 

    street_0.houses.push(house_a);
    street_0.houses.push(house_e);

    street_1.houses.push(house_d);

    //house_d is on both streets
    //street_0.houses.push(house_d);

    let mut moo_baan_0: MooBaan = MooBaan::new(01, "PuntCoolVille 88".to_string());

    moo_baan_0.streets.push(street_0);
    moo_baan_0.streets.push(street_1);

    let mut kv_company: DevelopmentCompany = DevelopmentCompany::new(1, "KV_Dev_Corp".to_string());
    kv_company.moo_baans.push(moo_baan_0);

    let mut big_corp_parent: HoldingCompany = HoldingCompany::new(1, "Big Corp".to_string());
    big_corp_parent.subsidiaries.push(kv_company);

    //Test pull back from Vec
    let company_structure = serde_json::to_string(&big_corp_parent)?;
    println!("{}", company_structure);

    Ok(())
}
