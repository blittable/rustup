#![allow(unused_variables)]

use colored::*;

fn main() {

    let color = [0, 128, 0]; 

    fn amaze<F>(f:  F) -> i32 
    where F: Fn(i32) -> i32 {
        f(1)
    }

    let k = |x| x*2;

    assert_eq!(amaze(k), 2);
    println!("{}", amaze(k).to_string().green());
}
