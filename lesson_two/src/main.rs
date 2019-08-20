fn main() {
    //We declare a variable and assign it a value
    let x = 10;
    let mut x: i32 = 10;
    println!("Value of x: {:?}", x);
    x = 3;
    println!("Value of x: {:?}", x);
}

struct Dog {
    id: u8,
    breed_id: u32,
    name: String,
}

fn another() {
    println!("Foo");
}
