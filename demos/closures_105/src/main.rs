/**
 * Closures 104 
 * Fn, FnOnce, and FnMutable 
 *
 * Source: https://github.com/eliovir/rust-examples
 * The differences here are import, especially for concurrency
 * and shared state.  Think: mutex, thread, options setting
 */

fn ten_times<F>(f: F)
where
    F: Fn(i32),
{
    let mut i = 0;
    while i < 10 {
        f(i);
        i += 1;
    }
}

fn not_ten_times<F>(mut f: F)
where
    F: FnMut(i32),
{
    let mut i = 0;
    while i < 10 {
        f(i);
        i += 1;
    }
}

fn not_even_ten_times<F>(f: F)
where
    F: FnOnce(i32),
{
    let i = 0;
    f(i);
}

fn main() {
    ten_times(|j| println!("hello, {}", j));
    not_ten_times(|j| println!("hello, {}", j));
    not_even_ten_times(|j| println!("hello, {}", j));
}
