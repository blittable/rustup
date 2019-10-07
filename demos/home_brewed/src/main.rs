#[allow(unused_mut)]
#[allow(unused_variables)]
#[allow(dead_code)]

fn main() {
    let mut x = 9;
    let mut c = || x * 2;

    let result_1 = flexi_function_fnonce(&c);
    println!("Result 2: {:?}", result_1);

    let result_2 = flexi_function_fn(&c);
    println!("Result 2: {:?}", result_2);

    let result_3 = flexi_function_fnmut(&c);
    println!("Result 3: {:?}", result_3);

    let mut x: usize = 1;
    {
        let add_two_to_x = || x += 2;
        do_twice(add_two_to_x);
    }

    assert_eq!(x, 5);
}

fn flexi_function_fnonce(f: impl FnOnce() -> i8) -> i8 {
    2 * &f()
}

fn flexi_function_fn(f: impl Fn() -> i8) -> i8 {
    2 * &f();
    2 * &f()
}

fn flexi_function_fnmut(mut f: impl FnMut() -> i8) -> i8 {
    f();
    f()
}

fn do_twice<F>(mut func: F)
where
    F: FnMut(),
{
    func();
    func();
}
