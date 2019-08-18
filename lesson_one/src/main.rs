fn main() {

    let _name = "Nook";
    let _name_with_description = add_description(_name.to_string()); 

    println!("{}", _name_with_description);
}

fn add_description(name: String) -> String {
    return format!("{} {}", name, "is great!");
}
