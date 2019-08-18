fn main() {
    let name = "Nook";
    let name_with_description = set_description(name); 
    println!("Hello {}" , name_with_description);
}

fn set_description(name : &str) -> String {
    // let mut name_to_owned = String::with_capacity(name.len());
    // for c in name.chars() {
    //   if c != ' ' {
    //      name_to_owned.push(c);
    //   }
    // }

    let new_owned_string = name.to_owned() + " is Great!";
    new_owned_string
}

