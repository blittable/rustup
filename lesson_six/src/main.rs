// A function which takes a closure and returns an `Vec<i32>`.
fn doubled_vec_closure<F>(f: F, array: [i32; 3]) -> Vec<i32>
where
    // The closure takes an `[i32; 3]` and returns an `Vec<i32>`.
    F: Fn([i32; 3]) -> Vec<i32>,
{
    f(array)
}

fn doubled_tuple_closure<F>(f: F, array: [i32; 3]) -> (i32, i32, i32)
where
    // The closure takes an `[i32; 3]` and returns an `tubles`.
    F: Fn([i32; 3]) -> (i32, i32, i32),
{
    f(array)
}

fn doubled_vec_closure_mut<F>(mut f: F)
where
    F: FnMut(),
{
    f();
}

fn doubled_vec_closure_once<F>(f: F, array: [i32; 3]) -> Vec<i32>
where
    // The closure takes an `[i32; 3]` and returns an `Vec<i32>`.
    F: FnOnce([i32; 3]) -> Vec<i32>,
{
    f(array)
}

fn main() {
    println!("Hello, Closure!");

    // Fn
    let a = [1, 2, 3];
    let func_a = |num: [i32; 3]| -> Vec<i32> { num.iter().map(|&x| x * 2).collect() };
    func_a(a);
    func_a(a);
    let doubled = doubled_vec_closure(func_a, a);
    assert_eq!(vec![2, 4, 6], doubled);
    println!("Doubled vector by Fn: {:?}", doubled);

    let tuple_func_a = |num: [i32; 3]| -> (i32, i32, i32) { (num[0] * 2, num[1] * 2, num[2] * 2) };
    let doubled_tubple = doubled_tuple_closure(tuple_func_a, a);
    assert_eq!((2, 4, 6), doubled_tubple);
    println!("Doubled tuple by Fn: {:?}", doubled_tubple);

    // FnMut
    let mut a2 = [1, 2, 3];
    {
        let mut double_a2 = || {
            a2.iter_mut().for_each(|x| *x *= 2);
        };
        double_a2();
        doubled_vec_closure_mut(double_a2);
        // Try to change new func passed in closure
        let divide_a2 = || {
            a2.iter_mut().for_each(|x| *x /= 2);
        };
        doubled_vec_closure_mut(divide_a2);
        assert_eq!(vec![2, 4, 6], a2);
        println!("Doubled vector by FnMut: {:?}", a2);
    }

    // FnOnce
    let a3 = [1, 2, 3];
    let doubled3 = doubled_vec_closure_once(
        |num: [i32; 3]| -> Vec<i32> { num.iter().map(|&x| x * 2).collect() },
        a3,
    );
    assert_eq!(vec![2, 4, 6], doubled3);
    println!("Doubled vector by FnOnce: {:?}", doubled3);

    println!("");
}
