fn main() {
    let name = "Rustican";
    let function_result = greetings(name);
    println!("{}", function_result);
}

fn greetings(name: &str) -> String {
    "Hello ".to_string() + name
}