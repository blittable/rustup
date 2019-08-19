fn main() {

    let name = "Nook";
    let name_with_description = add_description(name.to_string()); 

    println!("Name with description: {:?}", name_with_description);
}

fn add_description(name: String) -> String {

    let mut result: String = name;
    result.push_str(" is great!");
    
    result
}