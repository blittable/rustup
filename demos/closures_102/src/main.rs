#![allow(unused_variables)]
use colored::ColoredString;
use colored::Colorize;

fn main() {
    simple_closure();
    hello_closures_in_technicolor();
    anonymous_functions();
    hello_closures_in_technicolor();
    anonymous_functions();
}

//Point One: Closures look *a lot* like functions
//fn  plus_one_v1   (x: i32) -> i32 { x + 1 }
//let plus_one_v2 = |x: i32| -> i32 { x + 1 };
//let plus_one_v3 = |x: i32|          x + 1  ;

//Point Two: They can be invoked as anonymous functions
//However, variable x is 'captured' so it's an exception to the rule
fn simple_closure() {
    let names = ["John".to_string(), "Jim".to_string(), "Mary".to_string()];

    //For makes for sense here, but...
    for x in names.iter().map(|y| y) {
        println!("{}", x);
    }
}



fn simple_closure_invoked() {
    let x = "mycos".to_string();

    let simple_closure = move |x: String| {
        let joined: String = [x, " rocks".to_string()].concat();
        println!("{}", joined);
    };

    simple_closure("Mycos".to_string());
}

fn hello_closures_in_technicolor() {
    let green = "*******************".green();
    let yellow = "******MYCOS*******".yellow();
    let blue = "*******************".blue();

    let joined_separator = |x: &ColoredString, y: &ColoredString, z: &ColoredString| {
        [x.to_string(), y.to_string(), z.to_string()].concat();
        println!("{}{}{}", x, y, z);
    };

    joined_separator(&green, &yellow, &blue);
}

//Typical Usage One: Iteration with side-effect
fn anonymous_functions() {
    //Copy Trait!
    let some_array = [2, 4, 6, 8, 10];

    //Possible - but pointless
    for z in some_array.iter().map(|y| y) {
        println!("Closure Value: {:?}", z);
    }

    //Foreach is more common
    for z in some_array.iter() {
        println!("For Loop Value: {:?}", z);
    }

    //Typical adding a value
    //Note we are NOT mutating the value here
    for z in some_array.iter().map(|y| y + 1) {
        println!("Closure Value: {:?}", z);
    }
}

fn pretty_separator() {
    let green = "*******************".green();
    let yellow = "*******************".yellow();
    let blue = "*******************".blue();

    let joined_separator = [
        green.to_string(),
        "\n".to_string(),
        yellow.to_string(),
        "\n".to_string(),
        blue.to_string(),
    ]
    .concat();

    println!("{}", joined_separator);
}
