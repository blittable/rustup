fn main() {

    let name = "Nook";
    let name_with_description = add_description(name); 

    println!("Name with description: {:?}", name_with_description);
}

fn add_description(name: &str) -> String {

    //add "is great!" to the name 
    format!("{} is great!", name)
}
