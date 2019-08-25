//Write some code that does the following:
//1: uses an Option<T>
//2: uses Result<T, E> type
//3: uses 'match'

fn calculate_circle_area(rad: f32) -> Option<f32> {
    if rad < 0.0 {
        None
    } else {
        const PI: f32 = 3.14159;
        Some(PI * rad.powf(2.0))
    }
}

fn division(dividend: i32, divisor: i32) -> Result<i32, String> {
    if divisor == 0 {
        Err(String::from("Divide by zero exception!"))
    } else {
        Ok(dividend / divisor)
    }
}

fn main() {
    let area = calculate_circle_area(1.0);

    match area {
        None => println!("Radian value must be equal or greater than zero"),
        Some(a) => println!("Area is : {:?}", a)
    }

    match division(3, 1) {
        Ok(r) => println!("Result is : {:?}", r),
        Err(err) => println!("Error: {:?}", err)
    }

    match division(3, 0) {
        Ok(r) => println!("Result is : {:?}", r),
        Err(err) => println!("Error: {:?}", err)
    }
}
