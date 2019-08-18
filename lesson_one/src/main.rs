fn main() {
    let name = "iamkhwan";
    
    println!("Hello World!");
    hello(name);
}

fn hello(name: &str) {
    println!("Who am i : {}", name);
}