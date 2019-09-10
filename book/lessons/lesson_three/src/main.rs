//Basics of Traits - Homework

//Create a struct that has String field
    struct Ant {
        specie: String,
        sizeInMilimeter: u16,
    }
   

    //Create a trait that returns a String
    trait LifeThing {
        fn Communicate_With_Other(&self) -> String;
    }

    //Implement the trait for your struct
    impl LifeThing for Ant {

        fn Communicate_With_Other(&self) -> String {
            "...___...".to_string()
        }

    }

    impl LifeThing for String {
        fn Communicate_With_Other(&self) -> String {
            self.to_string()
        }
    }

fn main() {
    //Call it from main()
    let ant1: Ant = Ant {
        specie: "Black House Ant".to_string(),
        sizeInMilimeter: 3,
    };

    let message_from_ant = ant1.Communicate_With_Other(); // <-- This works b/c the SayHello trait is implemented for FriendlyGuy

    println!("Message from ant: {:?}", message_from_ant);
    //An alternative way to do the same thing - but there is no 'anonymous' type that can be pushed here - it must be a friendly_guy (generics to come!)

    println!(
        "Message from anonymous Ant: {:?}",
        self::LifeThing::Communicate_With_Other(&ant1)
    );


    //Extra Credit:
    // 1) create a String value
    let message1 = "Do not read me!".to_string();
    // 2) print the same string value returned from the trait above, but without using a struct, hint ::
    
    println!(
        "Message from anonymous LifeThing: {:?}",
        self::LifeThing::Communicate_With_Other(&message1)
    );

}
