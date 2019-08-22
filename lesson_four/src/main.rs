//Write some code that does the following:
//1: uses an Option<T>
//2: uses Result<T, E> type
//3: uses 'match'

use std::num::ParseIntError;

// uses Result<T, E> type and match
fn is_number_check_error(number_str: &str) -> Result<i32, ParseIntError> {
    match number_str.parse::<i32>() {
        Ok(n) => Ok(n),
        Err(err) => Err(err),
    }
}

// uses an Option<T>, Result<T, E> and match
fn get_even_number(number_str: &str) -> Result<Option<i32>, String> {
    let check_number = is_number_check_error(number_str);
    match check_number {
        Ok(n) => {
            if n % 2 == 0 {
                Ok(Some(n))
            } else {
                Ok(None)
            }
        }
        Err(err) => Err(format!("Parse {:?} char error: {:?}", number_str, err)),
    }
}

// uses Result<T, E> type and match
fn sort_filter_even_number_string(check_str: &str) -> Result<String, String> {
    let mut result: String = String::new();
    let mut error_str: String = String::new();
    let mut chars: Vec<char> = check_str.chars().collect();
    chars.sort();

    for c in chars {
        let even_number = get_even_number(&c.to_string());
        match even_number {
            Ok(n) => match n {
                Some(n_value) => result.push_str(&n_value.to_string()),
                None => (),
            },
            Err(err) => {
                if error_str.len() > 0 {
                    error_str.push_str(&format!(", {}", err));
                } else {
                    error_str.push_str(&err);
                }
            }
        }
    }

    if error_str.len() > 0 {
        Err(error_str)
    } else if result.len() > 0 {
        Ok(result)
    } else {
        Err("Not found any even number string!".to_string())
    }
}

fn main() {
    let test_string: String = "abc1".to_string();
    println!("\n> Input string: {:?}", test_string);
    let result: Result<String, String> = sort_filter_even_number_string(&test_string);
    assert_eq!(result.is_ok(), false);
    match result {
        Ok(r) => println!("Sort and filter only even number string: {:?}", r),
        Err(err) => println!("Error: {}", err),
    }

    let test_string: String = "135".to_string();
    println!("\n> Input string: {:?}", test_string);
    let result: Result<String, String> = sort_filter_even_number_string(&test_string);
    assert_eq!(result.is_ok(), false);
    match result {
        Ok(r) => println!("Sort and filter only even number string: {:?}", r),
        Err(err) => println!("Error: {}", err),
    }

    let test_string: String = "6234581".to_string();
    println!("\n> Input string: {:?}", test_string);
    let result: Result<String, String> = sort_filter_even_number_string(&test_string);
    assert_eq!(result.is_ok(), true);
    match result {
        Ok(r) => {
            assert_eq!(r, "2468");
            println!("Sort and filter only even number string: {:?}", r);
        }
        Err(err) => println!("Error: {}", err),
    }

    println!("");
}
