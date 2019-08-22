//Basics of Traits - Homework

//Create a struct that has String field
struct Person {
    name: String
}

//Create a trait that returns a String
trait Speak {
    fn speak(&self) -> String;
}

//Implement the trait for your struct
impl Speak for Person {
    fn speak(&self) -> String {
        format!("Hello! I am {}.", self.name)
    }
}

//Extra Credit: Implement the trait for String so that it can be called later.
impl Speak for String {
    fn speak(&self) -> String {
        format!("Hello! I am {}.", self)
    }
}


fn main() {

    let kevin: Person = Person {
        name: "Kevin".to_string()
    };

    //Call it from main()
    println!("Kevin says: {:?}", kevin.speak());

    //Extra Credit:
    // 1) create a String value
    // 2) print the same string value returned from the trait above, but without using a struct, hint ::
    let anon = "XXXXXXXX".to_string();
    println!("Anon says: {:?}", self::Speak::speak(&anon));
}