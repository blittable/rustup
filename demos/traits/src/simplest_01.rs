#[allow(dead_code)]

fn main() {

    //The simplest cases for traits
    let kwan: FriendlyGuy = FriendlyGuy {
        name: "Kwan".to_string(),
    };

    // CASE 1: A trait (Friendly Guy) is implemented by the a struct (Friendly Guy)
    let message_from_kwan = kwan.say_hello(); 
    println!("Message from Kwan: {:?}", message_from_kwan);

    // CASE 2: A trait is invoked trait::function::(trait-implementor)  
    println!(
        "Message from Kwan, trait::function::(implementor): {:?}",
        SayHello::say_hello(&kwan)
    );
}

struct FriendlyGuy {
    name: String,
}

//The simplest trait
trait SayHello {
    fn say_hello(&self) -> String;
}

//Implementing the trait for the type gets
//us .say_hello()  method syntax
impl SayHello for FriendlyGuy {
    fn say_hello(&self) -> String {
        "Hello!".to_string()
    }
}
