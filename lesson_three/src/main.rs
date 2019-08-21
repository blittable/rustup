//Basics of Traits - Homework
    struct Basketball { 
        name: &'static str,             
    }

    trait Player {        
        fn new(name: &'static str) -> Self;
        
        fn name(&self) -> &'static str;        

        fn talk(&self) {           
        }    
    }

    impl Basketball {    

        fn take_action(&mut self) {            
            println!("{} is playing basketball", self.name());
        }
    }

    impl Player for Basketball {
        
        fn new(name: &'static str) -> Basketball {
            Basketball { name: name}
        }

        fn name(&self) -> &'static str {
            self.name
        }       
        
        fn talk(&self) {            
            println!("My name is {}", self.name);
        }
    }

fn main() {

    //Create a struct that has String field

    //Create a trait that returns a String

    //Implement the trait for your struct

    //Call it from main()

    //Extra Credit:
    // 1) create a String value
    // 2) print the same string value returned from the trait above, but without using a struct, hint ::    

    let mut Krit: Basketball = Player::new("Krit");
 
    Krit.talk();
    Krit.take_action();    
}
