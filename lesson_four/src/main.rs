//Write some code that does the following:
//1: uses an Option<T>
//2: uses Result<T, E> type
//3: uses 'match'

use std::num::ParseFloatError;

struct PurchaseOrder {
    item_name: String,
    price: f32,
    qty: Option<String>,
}

fn calculate_unit_price(order: PurchaseOrder) -> Result<Option<f32>, ParseFloatError>  {
    let f_num: Option<f32> = match order.qty {
        None => None,
        Some(s) => Some(match s.parse::<f32>() {
            Ok(n) => n,
            Err(err) => return Err(err),
        }),
    };

    Ok(match f_num {
        None => None,
        Some(i) => Some(order.price / i),
    }) 
}

fn main() {
    let order :PurchaseOrder = PurchaseOrder {
        item_name: "Coke".to_string(),
        price: 229.0,
        qty: Some("20".to_string()),
    };

    println!("You buy {} for {:?} item", order.item_name, order.qty);

    match calculate_unit_price(order) {
        Ok(p) => Ok(match p {
            Some(i) => println!("You pay {} per item", i),
            None => println!("Cannot calcurate unit price bacause no number of item"),
        }),
        Err(err) => Err(println!("Error on calculate: {:?}", err)),
    };

    //Extra:: Make it error!!!
    println!("");
    println!("Let make an error");
    let error_order :PurchaseOrder = PurchaseOrder {
        item_name: "Bad Coke".to_string(),
        price: 100.0,
        qty: Some("Nope".to_string()),
    };
    println!("You cheat for {}", error_order.item_name);

    match calculate_unit_price(error_order) {
        Ok(p) => Ok(match p {
            Some(i) => println!("You pay {} per item", i),
            None => println!("Cannot calcurate unit price bacause no number of item"),
        }),
        Err(err) => Err(println!("Error on calculate because you cheat: {:?}", err)),
    };
}

