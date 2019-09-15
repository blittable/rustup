//Basics of Traits

fn main() {
    //The simplest case
    let kwan: FriendlyGuy = FriendlyGuy {
        name: "Kwan".to_string(),
    };

    let message_from_kwan = kwan.say_hello(); // <-- This works b/c the SayHello trait is implemented for FriendlyGuy
    println!("Message from Kwan: {:?}", message_from_kwan);

    //An alternative way to do the same thing - but there is no 'anonymous' type that can be pushed here - it must be a friendly_guy (generics to come!)
    println!(
        "Message from anonymous Kwan: {:?}",
        self::SayHello::say_hello(&kwan)
    );
}

struct FriendlyGuy {
    name: String,
}

//The simplest trait
trait SayHello {
    fn say_hello(&self) -> String;
}

//A 'default' trait implementation
impl SayHello {
    fn say_hello(&self) -> String {
        "Hello!".to_string()
    }
}

//Implementing the trait for the type gets
//us .say_hello()  method syntax
impl SayHello for FriendlyGuy {
    fn say_hello(&self) -> String {
        "Hello!".to_string()
    }
}

fn trait_for_free() {
    //We often get trait implementations for FREE
    //with the derive macro
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let origin = Point { x: 0, y: 0 };

    println!("The origin is: {:?}", origin);
}
