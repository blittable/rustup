fn main() {

    let name = "Nook";
    //let name_with_description = &name;

    let result = Add_XIsGreat(name.to_string());
    println!("{}", result);
}

fn Add_XIsGreat(who: String) -> String
{
    let message = " is great.".to_string();
    return format!("{} {}", who, message);
}



