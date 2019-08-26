#[inline]
pub fn fibonacci(n: i32) -> i32 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
