//Basics of Traits - Homework

fn main() {
    //Call it from main()
    let khwan: School = School {
        name: String::from("Mycostech")
    };

    let msg = khwan.study();
    println!("Result: {}", msg);

    //Extra Credit:
    // 1) create a String value
    let extra = "Mycostech 2".to_string();

    // 2) print the same string value returned from the trait above, but without using a struct, hint ::
    println!(
        "Result : {}",
        self::Study::study(&School { name: extra })
    ); 
}

//Create a struct that has String field
struct School {
    name: String
}

//Create a trait that returns a String
trait Study {
    fn study(&self) -> String; 
}

//Implement the trait for your struct
impl Study for School {
    fn study(&self) -> String {
        let msg = " Rust Programming";
        let mut my_new_str = self.name.clone();
        my_new_str.push_str(msg);
        my_new_str        
    }
}