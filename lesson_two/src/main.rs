//Playing with immutable and borrowing

fn main() {

    let mut nn = 10;
    nn = 12;
    println!("nn is {}", nn);


    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // integer not clone 
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    //with function 
    /// first, string
    let s = String::from("hello"); 
    takes_ownership(s);
    // println!("string is  {}", s); error

    /// second, int
    let num = 5;
    makes_copy(num);
    println!("num is {}", num);

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}