fn main() {
    let name = "Nook";
    println!("{}", print_name_is_handsome(name));
}

fn print_name_is_handsome(name: &str) -> String {
   let mut new_str = name.to_string();
    new_str.push_str(" is handsome");

    new_str
}