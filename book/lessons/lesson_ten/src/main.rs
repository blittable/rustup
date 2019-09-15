//Homework:
//Create a simple function, and writing a failing unit test
//You can modify and/or delete the samples here

fn main() {    
    println!("## Test Homework ##");
}

#[test]
fn it_works() {
    assert_ne!("Khwan", "Khwan");
}

#[test]
fn it_fail() {
    assert_eq!("Mocha", take_a_cup_of_coffee());
}

fn take_a_cup_of_coffee() -> String {
    "Latte".to_string()
}

#[test]
fn it_hello() -> Result<(), String> {
    if ((5+5) == 9) {
        Ok(())
    }
    else {
        Err(String::from("It's ten !!!"))
    }
}