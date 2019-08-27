#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(unused_mut)]

fn main() {
    closure_captures_borrow();
}

fn closure_captures_borrow() {
    //non-copy
    let mut name: String = "Mycos".to_string();

    //println!("I'm just a name: {}", &name);

    //The value is moved into the closure
    let name_joiner_closure = |name_parameter| name;

    println!(
        "Captured Value: {}",
        name_joiner_closure(&" rocks!".to_string()),
    );

    //Cannot name was consumed
    //println!("Can we still reference? {}", name);

    let haystack = vec![1, 2, 3];
    let contains = |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));
}
