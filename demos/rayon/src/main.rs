use rayon::prelude::*;
use std::collections::hash_map;
use std::collections::hash_map::RandomState;
use std::hash::BuildHasher;
use std::hash::{BuildHasherDefault, Hasher};
use std::time::{Duration, Instant};

const ITERATIONS: i32 = 100000000;

fn main() {
    println!("Calling Function with Rayon parallel!");

    let start = Instant::now();
    let largish_array = get_squareables(ITERATIONS);
    let x = sum_of_squares(&largish_array);
    let duration = start.elapsed();
    println!("Time elapsed in sum_of_squares() is: {:?}", duration);

    let start = Instant::now();
    let largish_array = get_squareables(ITERATIONS);
    let x = sum_of_squares_with_rayon(&largish_array);
    let duration = start.elapsed();
    println!(
        "Time elapsed in sum_of_squares_with_rayon() is: {:?}",
        duration
    );

    let start = Instant::now();
    let largish_array = get_hashables(ITERATIONS);
    let x = mega_hash(&largish_array);
    let duration = start.elapsed();
    println!("Mega Hash() is: {:?}", duration);

    let start = Instant::now();
    let largish_array = get_hashables(ITERATIONS);
    let x = mega_hash_with_rayon(&largish_array);
    let duration = start.elapsed();
    println!("Mega Hash with Rayon() is: {:?}", duration);
}

fn sum_of_squares_with_rayon(input: &Vec<f64>) -> f64 {
    input
        .par_iter() // <-- just change that!
        .map(|i| i * i.sqrt().sin() / 3.0)
        .sum::<f64>()
}

fn sum_of_squares(input: &Vec<f64>) -> f64 {
    input.iter().map(|i| i * i.sqrt().asin() / 3.0).sum::<f64>()
}

fn mega_hash(input: &Vec<i64>) -> usize {
    use std::collections::HashMap;

    input
        .iter() // <-- just change that!
        .map(|i| {
            let s = RandomState::new();
            let mut map: HashMap<u64, u64> = HashMap::with_hasher(s);
            (map.len())
        })
        .sum::<usize>()
}

fn mega_hash_with_rayon(input: &Vec<i64>) -> usize {
    use std::collections::HashMap;

    input
        .par_iter() // <-- just change that!
        .map(|i| {
            let s = RandomState::new();
            let mut map: HashMap<u64, u64> = HashMap::with_hasher(s);
            (map.len())
        })
        .sum::<usize>()
}

fn get_squareables(terminus: i32) -> Vec<f64> {
    (0..terminus).map(|x| f64::from(x)).collect()
}

fn get_hashables(terminus: i32) -> Vec<i64> {
    (0..terminus).map(|x| i64::from(x)).collect()
}
