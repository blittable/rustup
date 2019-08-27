#![allow(unused_variables)]
use rand::prelude::*;

fn main() {
    //cool
    if random() {
        // generates a boolean
        println!("Heads!");
    }

    //wishful thinking
    match random() {
        1 => println!("It's one!"),
        2 => println!("It's two!"),
        i => println!("It's other {}!", i),
    }
}
