//Basics of Traits - Homework

//Create a struct that has String field
struct Employee{
    name: String
}
//Create a trait that returns a String
trait Responsibility{
    fn work(&self) -> String;
}

//Implement the trait for your struct
impl Responsibility for Employee{
    fn work(&self) -> String{
        self.name.to_owned() + " is working."
    }
}

fn main() {
    //Call it from main()
    let kanoon: Employee = Employee{
        name : "Marisa".to_string()
    };
    println!("{:?}", kanoon.work());

    //Extra Credit:
    // 1) create a String value
    // 2) print the same string value returned from the trait above, but without using a struct, hint ::
    println!(
        "Message from anonymous: {:?}",
        self::Responsibility::work(&kanoon)
    );
}