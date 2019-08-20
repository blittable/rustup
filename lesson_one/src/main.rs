fn main() {

    let name: &str = "Aong";
    let con_name: String = contatenate(name);
    println!("My name is {}", con_name);
}

fn contatenate(x: &str) -> String {
    let title: &str = "Mr.";
    let result: String = format!("{} {}", title, x);
    return result;
}