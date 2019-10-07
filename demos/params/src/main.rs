fn main() {
    println!("Hello, world!");
}

fn doit<T, F>(t: T, f: F)
where
    T: FnOnce,
{
    f();
}
