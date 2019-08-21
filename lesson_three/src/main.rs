//Basics of Traits - Homework

fn main() {

    //Call it from main()
    let toyota: Car = Car {
        name: "Toyota".to_string(),
    };

    let toyota_speed = toyota.forward_speed();
    println!("Forward speed of {}: {:?}", toyota.name ,toyota_speed);

    //Extra Credit:
    // 1) create a String value
        let too_fast = "Too fast".to_string();
    // 2) print the same string value returned from the trait above, but without using a struct, hint ::
  println!(
        "{:?}",
        self::Speed::forward_speed(&too_fast)
    );

}
    //Create a struct that has String field
struct Car {
    name: String,
}
    //Create a trait that returns a String
trait Speed {
    fn forward_speed(&self) -> String;
}
    //Implement the trait for your struct
impl Speed for Car {
    fn forward_speed(&self) -> String {
        "80 km/hr".to_string()
    }
}
impl Speed for String {
        fn forward_speed(&self) -> String {
            self.to_string()
        }
    }
