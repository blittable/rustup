
use std::io;

fn main() {

    println!("How old are you?");

    let mut age = String::new();
    io::stdin().read_line(&mut age)
        .expect("Failed to read line");
    println!("{}",call_age_text(age.trim().parse().unwrap()));
}
fn call_age_text(age:i32) -> std::string::String
{
    return format!("I'm {} years old.", age+1);
}

