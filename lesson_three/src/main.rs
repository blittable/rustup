//Basics of Traits - Homework

//Create a struct that has String field
struct Footballer {
    name: String,
    height: u8
}

//Create a trait that returns a String
trait Kick {
    fn kick(&self) -> String;
}

//Implement the trait for your struct
impl Kick for Footballer {
    fn kick(&self) -> String {
        let mut _name = self.name.clone();

        if self.height > 180 {
            _name.push_str(" overhead kick... Booomm!!");
            return _name.to_string();
        } else {
            _name.push_str(" kick... Booomm!!");
            return _name.to_string();
        }
    }
}

fn main() {

    //Call it from main()
    let _owen = Footballer {
        name: "Owen".to_string(),
        height: 173
    };
    println!("{}", _owen.kick());

    let _van_dijk = Footballer {
        name: "Virgil van Dijk".to_string(),
        height: 193
    };
    println!("{}", _van_dijk.kick());

    //Extra Credit:
    // 1) create a String value
    // 2) print the same string value returned from the trait above, but without using a struct, hint ::

}
