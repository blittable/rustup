fn main() {
    println!("\nPlaying with immutable and borrowing.");

    // We declare a variable and assign it a value
    let x = 1;
    println!("Value of x: {:?}", x);
    let mut x: i32 = 10;
    println!("Value of x: {:?}", x);
    x = 3;
    println!("Edited value of x: {:?}", x);

    // Scope test
    {
        // Change value of x in scope
        x = 9;
        let  y: i32 = 99;
        println!("Scope value of y: {:?}", y);
    }

    println!("Edited value of x: {:?}", x);
    // Try to print y outside scope
    //println!("Value of y: {:?}", y);

    // Object test
    // Owner test
    let my_dog = Dog {
        id: 1,
        breed_id: DogBreeds::Shisu as u32,
        name: String::from("Keke"),
    };

    // Not allow to update immutable variable
    // my_dog.breed_id = DogBreeds::Golden as u32;

    // Copy data to new owner
    let mut dog_owner = my_dog.clone();
    dog_owner.name = String::from("Keke updated");
    print_dog_info_owner(my_dog);
    print_dog_info_owner(dog_owner);
    // print_dog_info_owner(my_dog);

    // Borrower Test
    let mut my_dog2 = Dog {
        id: 2,
        breed_id: DogBreeds::Pom as u32,
        name: String::from("Momo"),
    };
    print_dog_info_borrow(&my_dog2);
    // Assign by reference
    let dog_borrow = &mut my_dog2;
    print_dog_info_borrow(&dog_borrow);
    dog_borrow.name = String::from("Momo updated");
    print_dog_info_borrow(&dog_borrow);
    print_dog_info_borrow(&my_dog2);

    another();
    println!("");
}

fn print_dog_info_owner(dog: Dog) {
    println!(
        "[Owner] - My dog has id: {:?}, breed: {:?} and name: {:?}",
        dog.id,
        match DogBreeds::get_dog_breed_name(dog.breed_id) {
            Some(b) => b,
            None => "Unknown",
        },
        dog.name
    );
}

fn print_dog_info_borrow(dog: &Dog) {
    println!(
        "[Borrow] - My dog has id: {:?}, breed: {:?} and name: {:?}",
        dog.id,
        match DogBreeds::get_dog_breed_name(dog.breed_id) {
            Some(b) => b,
            None => "Unknown",
        },
        dog.name
    );
}

enum DogBreeds {
    Shisu = 1,
    Pom,
    Golden,
}

impl DogBreeds {
    // fn get_dog_breed(value: u32) -> DogBreeds {
    //     match value {
    //         1 => DogBreeds::Shisu,
    //         2 => DogBreeds::Pom,
    //         3 => DogBreeds::Golden,
    //         _ => panic!("Unknown value: {}", value),
    //     }
    // }

    fn get_dog_breed_name(value: u32) -> Option<&'static str> {
        match value {
            1 => Some("Shisu"),
            2 => Some("Pom"),
            3 => Some("Golden"),
            _ => None,
        }
    }
}

impl Clone for Dog {
    fn clone(&self) -> Dog {
        Dog {
            id: self.id,
            breed_id: self.breed_id,
            name: self.name.to_string(),
        }
    }
}

struct Dog {
    id: u8,
    breed_id: u32,
    name: String,
}

fn another() {
    println!("Foo");
}
