//Lesson Five: Error Handling

use std::env;
use std::error;
use std::fmt;

type Result<T> = std::result::Result<T, CustomError>;

#[derive(Debug, Clone)]
struct CustomError;

fn check_age(result: Result<i32>) {
    println!("Checking Age...");
    match result {
        Ok(n) => println!("Age still on range! {}", n),
        Err(e) => println!("Show Error: {}", e),
    }
}

fn validate(age: i32) -> Result<i32> {    
    if age > 125 { Err(CustomError) } else { Ok(age) }            
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Age error is over!, it's should not over >> 125.")
    }
}

impl error::Error for CustomError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> { None }
}

// https://doc.rust-lang.org/rust-by-example/std_misc/arg/matching.html
fn main() {  
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            eprintln!("We need a command line argument!");
            return;
        }
        2 => {
            let cmd = &args[1];
            let i: i32 = match cmd.parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("error in parsing the command line argument at position 1");
                    return;
                }
            };
            // Check and validate age.
            check_age(validate(i));                         
        }
        _ => {
            eprintln!("Error in parameters");
        }
    };
}

