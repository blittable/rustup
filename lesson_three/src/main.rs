//Basics of Traits - Homework

fn main() {
    //Call it from main()
    let man_u: Club = Club {
        name: "Man U".to_string(),
        num_of_player: 33
    };

    let showing = man_u.play();
    println!("GOoooo, {}", showing);
    //Noo haha
    //Extra Credit: 
    // 1) create a String value
    // 2) print the same string value returned from the trait above, but without using a struct, hint ::

}

//Create a struct that has String field
struct Club {
    name: String,
    num_of_player: i32
}

 //Create a trait that returns a String
trait Matching {
    fn play(&self) -> String;
}

 //Implement the trait for your struct
impl Matching for Club {
    fn play(&self) -> String {
        return format!("Team {} play with {} players", self.name, self.num_of_player)
    }
}