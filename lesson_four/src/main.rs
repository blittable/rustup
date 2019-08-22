use std::io;
use modulo::Mod;

fn main() {
    let answer = rand::random::<u32>().modulo(60) + 1;
    let mut guessing_count = 0;
    println!("Let's play number guessing game~~");
    println!("You, guess number from 1 - 60 na~");

    loop {
        let mut guess: u32;

        loop {
            guess = match get_number() {
                None => { 
                    println!("Not an number");
                    0
                },
                Some(v) => v    
            };
            if guess != 0 {
                break;
            }
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
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            match input.trim_end().parse::<u32>() {
                Ok(v) => return Some(v),
                Err(__) => return None
            };
        },
        Err(_) => return None
    }
}