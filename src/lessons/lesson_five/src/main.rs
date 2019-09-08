use std::env;

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

            validate_age(i)
        }
        _ => {
            eprintln!("Error in parameters");
        }
    };
}

fn validate_age(age: i32) {
    println!("Checking Age");
    if age > 125 {
        panic! {
            println!("Age cannot be over 125 {:?} and your input age was: ", age)
        }
    }
}
