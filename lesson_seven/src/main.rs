pub mod exceptions;
pub mod models;
pub mod services;

use crate::models::*;
use crate::services::customer::traits::*;
use crate::services::customer::*;
use crate::services::sale::traits::*;
use crate::services::sale::*;

fn main() {
    let buyer_service = BuyerService::new();
    let mut order_service = OrderService::new();

    // Test order#1
    let buyer = buyer_service.load("Sala", 1.00).unwrap();
    order_service.load("test order#2".to_string(), buyer);
    order_service.add_item(OrderItem::new_data((
        "tea0".to_string(),
        "tea zero".to_string(),
        5,
        9.45,
    )));

    order_service.add_item(OrderItem::new_data((
        "cake".to_string(),
        "cake".to_string(),
        0,
        15.00,
    )));

    order_service.calculate();
    assert_eq!(order_service.order.is_ok(), false);
    order_service.print();

    // Test order#2
    let buyer = buyer_service.load("Kaka", 1.00).unwrap();
    order_service.load("test order#2".to_string(), buyer);
    order_service.add_item(OrderItem::new_data((
        "Pizza".to_string(),
        "Pizza".to_string(),
        5,
        9.45,
    )));

    order_service.calculate();
    assert_eq!(order_service.order.is_ok(), false);
    order_service.print();

    // Test order#3
    let buyer = buyer_service.load("Jason", 100.00).unwrap();
    order_service.load("test order#3".to_string(), buyer);
    order_service.add_item(OrderItem::new_data((
        "coke0".to_string(),
        "coke zero".to_string(),
        1,
        10.45,
    )));

    order_service.add_item(OrderItem::new_data((
        "coke".to_string(),
        "coke".to_string(),
        3,
        15.00,
    )));

    order_service.calculate();
    assert_eq!(order_service.order.is_ok(), true);
    match &order_service.order {
        Ok(result) => match &result {
            Some(o) => {
                assert_eq!(o.total, 55.45);
                assert_eq!(o.order_items.iter().count(), 2);
            }
            None => println!("Invalid order!"),
        },
        Err(err) => println!("Error: {:?}", err),
    }
    order_service.print();

    println!("");
}
