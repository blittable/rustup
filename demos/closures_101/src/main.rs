#![allow(unused_variables)]

fn main() {
    let nums = [1, 2, 3];

    let doubled: Vec<i32> = nums.iter().map(|&n| n*2).collect();
    println!("Our collected results for doubled: {:?}", doubled);
}