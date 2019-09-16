use std::num::ParseIntError;

fn main() {
    println!("Hello, world!");
}

fn parse_our_string_as_i32(our_string: &str) -> Result<i32, ParseIntError> {
            our_string.parse::<i32>()
}

#[test]
fn parse_string_test_success() {
    match parse_our_string_as_i32("32") {
        Ok(i) => println!("Success {:?}", i),
        Err(e) => println!("Error {:?}", e),
    };
}

#[test]
fn parse_string_test_fail() {
    let result = parse_our_string_as_i32("j");

    match parse_our_string_as_i32("j") {
        Ok(i) => println!("Success {:?}", i),
        Err(e) => println!("Error {:?}", e),
    };
}



