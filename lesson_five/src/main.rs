use std::env;
use std::error;
use std::fmt;

type Result<T> = std::result::Result<T, AgeOver125Error>;

// Define our error types. These may be customized for our error handling cases.
// Now we will be able to write our own errors, defer to an underlying error
// implementation, or do something in between.
#[derive(Debug, Clone)]
struct AgeOver125Error{
    message: String
}

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl fmt::Display for AgeOver125Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "{:?}", self)
        write!(f, "AgeOver125Error {{ kind: {:?}, message: {:?} }}","InvalidAge", self.message)
    }
}

// This is important for other errors to wrap this one.
impl error::Error for AgeOver125Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

fn validate_age_custom_error(age: i32) -> Result<i32> {
    println!("Checking Age");

    if age > 125 {
        Err(AgeOver125Error{
            message: format!("Age cannot be over 125 and your input age was: {:?}", age)
        })
    }else{
        Ok(age)
    }
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("Age: {} is valid", n),
        Err(e) => println!("Error: {}", e),
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
                Err(err) => {
                    eprintln!("Error in parsing the command line argument at position 1, {:?}", err);
                    return;
                }
            };

            // validate_age(i);
            print(validate_age_custom_error(i));
        }
        _ => {
            eprintln!("Error in parameters");
        }
    };
}

fn validate_age(age: i32) {
    println!("Checking Age");
    if age > 125 {
        panic!("Age cannot be over 125 and your input age was: {:?}", age);
    }else{
        println!("Age: {} is valid", age);
    }
}
