fn main() {
    let mut s = String::from("hello");
    let r1 = &s; 
    let r2 = &s; 
    println!("{} and {}", r1, r2);
    let r3 = &mut s; 
    println!("{}", r3);

    println!("--------------");
    println!("{}",s);
    change(&mut s);
    println!("{}",s);

    println!("--------------");

    let a = first_word(&s);
    println!("{}",a);
}

fn change(some_string: &mut String) {
    some_string.push_str(" , world");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b'r' {
            return &s[0..i];
        }
    }

    &s[..]
}
