
fn main() {

    let name = "Nook";
    let name_with_description = add_description(&name); 
    let result = add();

    println!("Name with description: {:?}", name_with_description);
    println!("Name with description: {:?}", result); 
}

fn add_description(name: &str) -> String {
    [name, " is great"].concat()
}


fn return_something() -> i32 {
    1
}


