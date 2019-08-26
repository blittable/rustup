#[allow(unused_mut)]
#[allow(unused_variables)]
#[allow(dead_code)]

//As a question:
//When I use a closure as an input parameter,
//I need to specify how the surrounding values
//are consumed by the closure.  The receiving
//functions must provide the trait bounds needed
//for my 'capture' scenario.

fn main() {
    let mut b = false;
    let x = &mut b;

    let mut c = || {
        *x = true;
    };
    // The following line is an error:
    // let y = &x;
    c();

    let z = &x;

    closure_tests();
}

fn closure_tests() {
    let owned_value = "Mycos".to_string();
    let static_ref_str_value = "Mycos";
    let owned_mut_value: String = "Mycos".to_string();

    let closure = move || {
        println!("{}", owned_value);
        println!("{}", static_ref_str_value);
        println!("{}", owned_mut_value);
    };

    apply_once(closure);
    //apply_once(closure);
    //apply_mut(closure);
}

fn apply_once<F>(f: F)
where
    F: FnOnce(),
{
    return f();
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
