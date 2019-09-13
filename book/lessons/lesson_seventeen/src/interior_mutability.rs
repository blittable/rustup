#[allow(dead_code)]
use std::rc::{Rc};
use std::cell::{RefCell, RefMut};


#[derive(Debug)]
struct FamilyMember {
    name: RefCell<String>,    // <- Change 1, the the mutable content needs to be mutable, so change &'static str (static lives forever and is immutable)
    is_parent: bool,
}

impl FamilyMember {
    pub fn new_rc(new_name: &'static str, parent: bool) -> Rc<FamilyMember> {
        Rc::new(FamilyMember {
            name: RefCell::new(new_name.to_string()),
            is_parent: parent,
        })
    }
}

struct Car {
    number: u16,
    make: &'static str,
    is_clean: bool,
    drivers: Vec<Rc<FamilyMember>>,
}

impl Car {
    pub fn new(new_number: u16, car_make: &'static str) -> Car {
        Car {
            number: new_number
            
            make: car_make,
            is_clean: false,
            drivers: Vec::<Rc<FamilyMember>>::new(),
        }
    }
}


fn count_drivers(car: & Car) -> usize {
    car.drivers.iter().fold(0, |cumul, x| cumul + (Rc::strong_count(x)/2)) // <- (Strong Count has a Self reference so 2 for every referant)
}

pub fn main() {
    //Create the family
    let bobby = FamilyMember::new_rc("Bob", false);
    let john = FamilyMember::new_rc("John", false);
    let connie = FamilyMember::new_rc("Connie", false);
    let chris = FamilyMember::new_rc("Chris", true);
    let laura = FamilyMember::new_rc("Laura", true);

    //Create the cars
    let mut mira = Car::new(0, "Mira");
    let mut subaru = Car::new(1, "Subaru");
    let mut ferrari = Car::new(1, "Ferrari");

    // Everyone can drive the Mira!
    // shared ownership with Rc<T> and Rc::clone
    mira.drivers.push(Rc::clone(&bobby));
    mira.drivers.push(Rc::clone(&john));
    mira.drivers.push(Rc::clone(&connie));
    mira.drivers.push(Rc::clone(&chris));
    mira.drivers.push(Rc::clone(&laura));

    // Only two drivers for Ferrari!
    ferrari.drivers.push(Rc::clone(&chris));
    ferrari.drivers.push(Rc::clone(&laura));

    println!("Mira driver count {}", count_drivers(&mira));
    println!("Ferrari driver count {}", count_drivers(&ferrari));


    println!("Bobby's name {}", &bobby.name.borrow());
    *bobby.name.borrow_mut() = "Kim".to_string();
    println!("Bobby's name {}", &bobby.name.borrow());

    drop(bobby);

    println!("Post drop bobby driver count {}", count_drivers(&mira));

}
