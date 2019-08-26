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

            match validate_age(i) {
                Ok(v) => {
                    println!("{} is still young!!", v);
                }
                Err(message) => {
                    println!("#ERROR#: {}, and your input age was: {}", message, i);
                }
            }
        }
        _ => {
            eprintln!("Error in parameters");
        }
    };
}

fn validate_age(age: i32) -> Result<i32, &'static str>{
    println!("Checking Age");
    if age > 125 {
        return Err("Age cannot be over 125");
    }
    Ok(age)
}
