fn main() {
    let name = String::from("iamkhwan");    
    let answer = hello(&name);
    println!("{}", answer);
}

fn hello(name: &String) -> String {    
    let result = format!("Who am i : {}", name);
    return result;
}