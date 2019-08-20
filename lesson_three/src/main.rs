//Basics of Traits - Homework

fn main() {
    //Call it from main()
    let me: Person = Person {
        name: "Vee".to_string(),
    };

    let message = me.say_hi();
     println!("Me said: {:?}", message);

    let vee = String::from("Vee");
    println!(
        "Anonymous said: {:?}",
        self::Greeting::say_hi(&vee)
    );
    //Extra Credit:
    // 1) create a String value
    // 2) print the same string value returned from the trait above, but without using a struct, hint ::
}

//Create a struct that has String field
struct Person {
    name: String,
}

//Create a trait that returns a String
trait Greeting {
    fn say_hi(&self) -> String;
}

impl Greeting for String {
    fn say_hi(&self) -> String {
        "Hi!".to_string()
    }
}

//Implement the trait for your struct
impl Greeting for Person {
    fn say_hi(&self) -> String {
        "Hi!".to_string()
    }
}