
//Function to support vec.
pub fn hello_vec() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}

//Function to support tuple.
pub fn cal_area(dimensions: (u32, u32)) -> u32 {    
    dimensions.0 * dimensions.1
}