fn main() {
    let target: &str;

    //Narrow our scope
    {
        let string1 = "abcd";
        let string2 = "xyzwkrp";

        target = longest(string1, string2);

        println!("The longest string is {}", target);
    }

    println!("The longest string is {}", target);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
