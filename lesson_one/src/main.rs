fn main() {
    let name = String::from("Nook");
    let name_with_description = add_description(name); 

    println!("hello world");
    println!("Name with description: {}", name_with_description);
}

// Take ownership from caller, update, and then give back ownership to caller
fn add_description(mut name: String) -> String {
    name.push_str(" is great!");
    name
}