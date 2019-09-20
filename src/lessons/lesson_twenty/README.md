# Lesson Twenty: Working with Threads 

## Objectives 
- Understand the fundamental of working with threads in Rust 


_______

## Overview 


Working with threads in Rust is fun.  RC<T> and it's associates as well as mutability and borrowing let's us safely run native threads in Rust.  

OO programmers most likely have seen or used threads in other languages, but typically on `green threads` fare supported in these languages.  That's a level of virtualization, and comes at a cost.

_______

## Getting Started with Threads in Rust


Syntactically, Rust threads take a function, and it is common to see them in-lined with a closure as follows:


```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```

[Running the above]: (https://asciinema.org/a/tyT7zWrIZq6YOe8dq8mvoLDmi)

_______

### Thread lifetimes

- Importantly, when a parent thread is dropped, a child thread continues
- The `.join()` function joins the thread of the current context and waits for its termination.    

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    drop(v); // oh no!

    handle.join().unwrap();
}
```

```rust
use std::thread;
use std::time::Duration;

fn main() {
    println!("");
    let split = |n| {
        for _i in 1..n {
            print!("*");
        }
        println!("");
    };

    split(45);
    thread_lifetime();
}

//Note non-deterministic output
fn thread_lifetime() {

    let original_thread = thread::spawn(|| {
        let _detached_thread = thread::spawn(|| {
            // Here we sleep to make sure that the first thread returns before.
            thread::sleep(Duration::from_millis(10));
            // This will be called, even though the JoinHandle is dropped.
            println!("♫ Still alive ♫");
        });
    });

    original_thread
        .join() // <- Waits for the associated thread to finish.
        .expect("The thread being joined has panicked");
    println!("Original thread is joined.");

    // We make sure that the new thread has time to run, before the main
    // thread returns.

    thread::sleep(Duration::from_millis(1000));
}
```

#### Move values into the Threads closure


```rust, no_run
let k = [1, 2, 3];
    thread::spawn(|| { let n = k[0]; });  // <- Compile Error
```

```rust, no_run
let k = [1, 2, 3];
    thread::spawn(move || { let n = k[0]; }); 
```


## Channels - passing data between threads

```rust
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

_______

## Rayon: A Crate for Parallelism

One of the most popular crates in the Rust ecosystem is [Rayon](https://crates.io/crates/rayon)

With the addition of the crate, and the a simple change of the iterators to `_par_iter()` single-threaded iteration can changed to use multiple threads.

Below are two benchmarks, one for floating point processing and another with a hash initialization at 100 million iterations.

 Function | Time 
 ---- | ---
sum_of_squares - fp | *738.896ms*
sum_of_squares_with_rayon -fp | *531.016ms*
mega_hash | *678.212ms*
mega_hash_with_rayon | *198.15ms*

```rust 
use rayon::prelude::*;
use std::collections::hash_map;
use std::collections::hash_map::RandomState;
use std::hash::BuildHasher;
use std::hash::{BuildHasherDefault, Hasher};
use std::time::{Duration, Instant};

const ITERATIONS: i32 = 100_000_000;


//This currently does not work with the Rust Playground as it pegs the CPU
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
```
























