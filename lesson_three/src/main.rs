//Basics of Traits - Homework

fn main() {
    //Create a struct that has String field

    //Create a trait that returns a String

    //Implement the trait for your struct

    //Call it from main()

    //Extra Credit:
    // 1) create a String value
    // 2) print the same string value returned from the trait above, but without using a struct, hint ::

    // *** Start implementation ***
    println!("Playing Basics of Traits.");

    // Call struct from main()
    let mut my_dog = Dog {
        breed: DogBreeds::Bulldog,
        name: "Batman".to_string(),
        energy: 3,
    };

    my_dog.show_status();

    my_dog.play();

    println!("{}", my_dog.respond("eat"));

    my_dog.eat();

    println!("{}", my_dog.respond("play"));

    println!("{}", my_dog.respond("sit"));

    for _time in 0..4 {
        my_dog.play();
    }

    my_dog.sleep();

    println!("{}", my_dog.respond("play"));

    my_dog.bark();

    my_dog.show_status();

    println!("{}", my_dog.get_energy_detail());

    // 1) create a String value
    // 2) print the same string value returned from the trait above, but without using a struct, hint ::
    println!(
        "Response message from anonymous Dog: {}",
        self::IPet::respond(&mut my_dog, "play")
    );

    println!(
        "Response message from anonymous Dog: {}",
        self::IPet::respond(&mut my_dog, "sit")
    );

    println!(
        "Response message from anonymous Dog: {}",
        self::IPet::get_energy_detail(&my_dog)
    );

    my_dog.show_status();

    println!("");
}

#[derive(Debug)]
enum DogBreeds {
    Unknown = 0,
    Shisu = 1,
    Pom,
    Golden,
    Bulldog,
}

trait IPet {
    fn show_status(&self);
    fn eat(&mut self);
    fn play(&mut self);
    fn sleep(&mut self);

    // Create a trait that returns a String
    fn get_energy_detail(&self) -> String;
    fn respond(&mut self, command: &'static str) -> String;
}

trait IDog {
    fn bark(&mut self);
}

// Implement the trait for struct
impl IPet for Dog {
    fn show_status(&self) {
        println!("Current status: {:?}", self);
    }

    fn eat(&mut self) {
        self.energy += 1;
        println!("{:?} dog eats.", self.name);
    }

    fn play(&mut self) {
        if self.energy > 0 {
            self.energy -= 1;
            println!("{:?} dog plays.", self.name);
        } else {
            println!("{:?} dog has not energy to play.", self.name);
        }
    }

    fn sleep(&mut self) {
        self.energy = 5;
        println!("{:?} dog sleeps.", self.name);
    }

    fn get_energy_detail(&self) -> String {
        format!(
            "{:?} has {:?} kcal points of energy left.",
            self.name, self.energy
        )
    }

    fn respond(&mut self, command: &'static str) -> String {
        if self.energy <= 0 {
            return format!(
                "{:?} dog has not energy to respond {:?} command.",
                self.name, command
            );
        }

        match command {
            "eat" => {
                self.eat();
                format!("{:?} dog got {:?} command.", self.name, command)
            }
            "play" => {
                self.play();
                format!("{:?} dog got {:?} command.", self.name, command)
            }
            _ => format!(
                "{:?} dog does not understand {:?} command.",
                self.name, command
            ),
        }
    }
}

// Implement the trait for struct
impl IDog for Dog {
    fn bark(&mut self) {
        if self.energy > 0 {
            self.energy -= 1;
            println!("{:?} dog barks.", self.name);
        } else {
            println!("{:?} dog has not energy to bark.", self.name);
        }
    }
}

// Create a struct that has String field
#[derive(Debug)]
struct Dog {
    breed: DogBreeds,
    name: String,
    energy: i32,
}
