use std::io;
use modulo::Mod;

fn main() {
    let answer = rand::random::<u32>().modulo(60) + 1;
    let mut guessing_count = 0;
    println!("Let's play number guessing game~~");
    println!("You, guess number from 1 - 60, for challenge, beat it under 7 times");

    loop {
        let guess: u32;

        loop {
            match get_number() {
                None => { 
                    println!("Not an number");
                },
                Some(v) => {
                    guess = v;
                    break;
                }
            };
        }

        guessing_count += 1;
        if guess < answer {
            println!("larger than this");
        } else if guess > answer {
            println!("smaller than this");
        } else {
            println!("Correct!! YEY!!");
            break;
        }
    }

    println!("You've guessed {} time(s).", guessing_count);
}

fn get_number() -> Option<u32>{
    let mut input = String::new();
    return match io::stdin().read_line(&mut input) {
        Ok(_) => {
            match input.trim_end().parse::<u32>() {
                Ok(v) => Some(v),
                Err(__) => None
            }
        },
        Err(_) => None
    }
}