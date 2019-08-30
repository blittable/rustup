extern crate lesson_seven;

//To support sample #2
use lesson_seven::customer::usa;
use lesson_seven::customer::thai;

//To support sample #3
use lesson_seven::customer::thai::hello;
use lesson_seven::customer::usa::goodbye;

//To support dependencies
use rand::prelude::*;

//Support Vectors & tuple.
pub mod utils;

fn main() {    
    print!("\n");

    //Syntax variations for crates and modules
    //--------------------
    //sampl1 #1
    println!("Sample #1 >> Hello in Thai: {}", lesson_seven::customer::thai::hello());
    println!("Sample #1 >> Goodbye in English: {}", lesson_seven::customer::usa::goodbye());    
    print!("\n");

    //sample #2
    println!("Sample #2 >> Hello in Thai: {}", thai::hello());
    println!("Sample #2 >> Goodbye in English: {}",usa::goodbye());
    print!("\n");

    //sample #3
    println!("Sample #3 >> Hello in Thai: {}", hello());
    println!("Sample #3 >> Goodbye in English: {}", goodbye());
    print!("\n");

    //Dependencies support    
    //--------------------
    print!("Dependencies support\n");
    print!("--------------------\n");
    let x: u8 = random();
    println!("{}", x);

    let mut rng = thread_rng();
    if rng.gen() {
        let x: f64 = rng.gen(); 
        let y = rng.gen_range(-10.0, 10.0);
        println!("x is: {}", x);
        println!("y is: {}", y);
        println!("Number from 0 to 9: {}", rng.gen_range(0, 10));
    }

    let distr = rand::distributions::Uniform::new_inclusive(1, 100);
    let mut nums = [0i32; 3];
    for x in &mut nums {
        *x = rng.sample(distr);
    }
    
    println!("Some numbers: {:?}", nums);    
    let arrows_iter = "➡⬈⬆⬉⬅⬋⬇⬊".chars();
    println!("Lets go in this direction: {}", arrows_iter.choose(&mut rng).unwrap());
    let mut nums = [1, 2, 3, 4, 5];
    nums.shuffle(&mut rng);
    println!("I shuffled my {:?}", nums);        

    //Vectors support
    //---------------
    print!("\n"); 
    print!("Vectors basic\n");
    print!("--------------------\n");
    utils::hello_vec();

    //Tuples support
    //--------------
    print!("\n"); 
    print!("Tuple basic\n");
    print!("--------------------\n");    
    let rect1 = (50, 150);
    let _value:u32 = utils::cal_area(rect1);
    println!("Tuple basic: {}",_value);
}
