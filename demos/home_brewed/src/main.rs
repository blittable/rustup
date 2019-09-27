#[allow(unused_mut)]
#[allow(unused_variables)]
#[allow(dead_code)]

//As a question:
//When I use a closure as an input parameter,
//I need to specify how the surrounding values
//are consumed by the closure.  

fn main() {
    closure_tests();
}

fn flexi_function(count: i8, f: &dyn Fn(i8) -> i8) -> i8 {
    println!("{}", f(count));
    count 
}

fn age_in_20<T>(current: T) -> T where T: std::ops::Add<Output=T> {
    current //TODO
}

fn closure_tests() {
    flexi_function(4, &age_in_20);
}

fn greet() {
   println!("Wonderful"); 
}

fn apply_once<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn apply_alot<F>(f: F)
where
    F: Fn(),
{
    return f();
}

fn apply_mut<F>(mut f: F)
where
    F: FnMut(),
{
    return f();
}
