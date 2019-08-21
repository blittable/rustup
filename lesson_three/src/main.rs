//===============================================================
//Basics of Traits - Homework 3 - borrowing and ownership, mostly
//===============================================================

//Struct
#[derive(Debug)]
struct Employee {
    name: String
}

//Trait
trait Company {    
  fn get_name(&self) -> String;
}

//Implement
impl Company for Employee {    
    fn get_name(&self) -> String {
        self.name.to_string()
        //self.name.clone()
    }                   
}

fn helper_as_trait<T: Company>(t: T) -> T { t }

fn main() {

    //1) Create a struct that has String field
    //2) Create a trait that returns a String
    //3) Implement the trait for your struct
    //4) Call it from main()    
    //Extra Credit:
    // 5) create a String value
    // 6) print the same string value returned from the trait above, but without using a struct, hint ::

    let _t = Employee {
        name: "Steve".to_string()
    };

    let new_t = helper_as_trait(_t);
    println!("Hello, world! {}", new_t.get_name());

    //let showname = _t.get_name();
    //println!("Hello, world! {}", showname);

    let department = Employee::get_name(&Employee { 
        name: "Software department".to_string() 
    });

    println!("{}", department)    
}
