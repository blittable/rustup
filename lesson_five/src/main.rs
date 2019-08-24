// //Homework: There's a simple command-line application in lesson_five that takes one parameter. 
// //If the parameter is great than 125, the program errors. Use a Result<T, E> to improve the error handling.
use std::env;
use std::error;
use std::fmt;

type Result<T> = std::result::Result<T, AgeError>;

#[derive(Debug, Clone)]
struct AgeError;

impl fmt::Display for AgeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid age")
    }
}
impl error::Error for AgeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
fn validate_age(age: i32) -> Result<bool>{
    if(age > 125) {
        Err(AgeError)
    }
    else{
        Ok(true)

    }
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
            let result = validate_age(i);
            match result{
                Ok(_i) => {
                        println!("Your validate age result is correct.");
                },
                Err(e) =>{
                    println!("Error - {}", e);
                }
            }
        }
        _ => {
            eprintln!("Error in parameters");
        }
    };
}
// fn validate_age(age: i32) {
//     println!("Checking Age");
//     if age > 125 {
//         panic! {
//             println!("Age cannot be over 125 {:?} and your input age was: ", age)
//         }
//     }
// }
