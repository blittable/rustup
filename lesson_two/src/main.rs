
fn main() {    
    //We declare a variable and assign it a value
    let x = 10;
    let mut x: i32 = 10;
    println!("Value of x: {:?}", x);
    x = 3;
    println!("Value of x: {:?}", x);

    //time to play..

    //Variable Scope
    let s = "hello";
    println!("1: {}",s);    
    {                      
        let s = "hello"; 
        println!("2: {}",s);       
    }  

    //String Type
    let mut s = String::from("hello");    
    s.push_str(",world");
    s.push_str(",+rust");
    println!("3: {}",s);

    //String Type and Variable Scope
    {
        let s = String::from("hello");
        //cannot assign twice to immutable variable
        //s = "hello1".to_string();
        //println!("4: {}",s);

        //cannot assign twice to immutable variable
        //let x = s;
        //println!("5: {}",x);        
    }    

    //Ownership and Functions    
    let  ss = String::from("hello2");

    takes_ownership(ss);

    let xx = 5;

    makes_copy(xx);

    //Return Values and Scope        
    println!("{}",give_ownership_and_return());

    let s1 = give_ownership_and_return();
    println!("give_ownership_and_return and return {}",s1);

    let s2 = takes_ownership_parameter_and_return(s1);
    println!("takes_ownership_parameter_and_return: {}",s2);
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}",some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}",some_integer);
}

fn give_ownership_and_return() -> String {    
    //let some_string = "xxx";
    let some_string = String::from("Give_ownership_and_return function");
    some_string
}

fn takes_ownership_parameter_and_return(a_string: String) -> String {
    a_string
}

struct Dog {
    id: u8,
    breed_id: u32,
    name: String,
}

fn another() {
    println!("Foo");
}
