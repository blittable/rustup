//Write some code that does the following:
//1: uses an Option<T>
//2: uses Result<T, E> type
//3: uses 'match'

use std::num::ParseIntError;

struct MycosFarm {
    number_of_pig: i32,
    exchange_pig_rate: Option<f32>
}

fn get_exchange_value(mycos_farm: MycosFarm) -> Option<f32> {
    match mycos_farm.exchange_pig_rate {
        None => None,
        Some(v) => Some(v * mycos_farm.number_of_pig as f32)
    }
}


fn call_ultra_man(power: &str) -> Result<i32, ParseIntError> {
    match power.parse::<i32>() {
        Ok(n) => Ok(n * 25 as i32),
        Err(err) => Err(err)
    }
}

fn main() {

    //1: uses an Option<T>
    println!("Hello, welcome to Option section");

    let my_farm: MycosFarm = MycosFarm {
        number_of_pig: 5,
        exchange_pig_rate: Some(2500.00)
    };

    let result = get_exchange_value(my_farm);

    match result {
        Some(i) => println!("Your pig value : {:?}", i),
        None => println!("There is an error")
    }

    //2: uses Result<T, E> type
    println!("Hello, welcome to Result section");

    match call_ultra_man("20") {
        Ok(n) => println!("My ultraman power is {}", n),
        Err(err) => println!("Error: {}", err)
    }
   
}

