//Basics of Traits - Homework

//Create a struct that has String field
struct Footballer {
    name: String
}

//Create a trait that returns a String
trait Kick {
    fn kick(&self) -> String;
}

//Implement the trait for your struct
impl Kick for Footballer {
    fn kick(&self) -> String {
        String::from("Kick... Booomm!!!")
    }
}

fn main() {

    //Call it from main()
    let _footballer = Footballer {
        name: String::from("Owen")
    };
    println!("{}", _footballer.kick());

    //Extra Credit:
    // 1) create a String value
    // 2) print the same string value returned from the trait above, but without using a struct, hint ::

}
