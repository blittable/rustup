fn main() {
    let name = "Nook";
    let name_with_description = add_description(&name);

    println!("\nName with description: {:?}\n", name_with_description);
}

fn add_description(name: &str) -> String {
    format!("{} is great", name)
}
