pub mod traits;

use crate::exceptions::ServiceException;
use crate::models::*;
use crate::services::sale::traits::OrderServiceTrait;
use uuid::Uuid;

// -- OrderService
#[derive(Debug)]
pub struct OrderService {
    pub order: Result<Option<Order>, ServiceException>,
    pub buyer: Buyer,
}

impl OrderServiceTrait for OrderService {
    fn new() -> OrderService {
        let mut default_buyer = Buyer::new();
        default_buyer.name = "A Cash Account".to_string();

        OrderService {
            order: Ok(None),
            buyer: default_buyer,
        }
    }

    fn load(&mut self, order_name: String, buyer: Buyer) {
        println!("\n> load order...");

        // Validate data
        if order_name == "" {
            self.order = Err(ServiceException {
                message: "order name is empty!".to_string(),
            });
            return;
        }

        // Create new order
        let new_order = Order {
            id: Uuid::new_v4(),
            name: order_name,
            buyer: buyer.name.to_owned(),
            total: 0.0,
            order_items: <Vec<OrderItem>>::new(),
        };

        self.buyer = buyer;
        self.order = Ok(Some(new_order));
    }

    fn add_item(&mut self, item: OrderItem) {
        println!("add item...");

        // Validate data
        if item.code == "" {
            self.order = Err(ServiceException {
                message: "Item code is empty!".to_string(),
            });
            return;
        }

        if item.qty <= 0 {
            self.order = Err(ServiceException {
                message: "Invalid Qty!".to_string(),
            });
            return;
        }

        match &mut self.order {
            Ok(result) => match result {
                Some(result) => result.order_items.push(item),
                None => println!("Not found any order!"),
            },
            Err(err) => println!("Error: {:?}", err),
        };
    }

    fn calculate(&mut self) {
        println!("calculate order...");

        match &mut self.order {
            Ok(result) => match result {
                Some(result) => {
                    result
                        .order_items
                        .iter_mut()
                        .for_each(|i: &mut OrderItem| i.price = i.unit_price * (i.qty as f32));

                    result.total = result.order_items.iter().map(|i| i.price).sum();

                    if result.total >= self.buyer.credit {
                        self.order = Err(ServiceException {
                            message: "Invalid credit!".to_string(),
                        });

                        match &self.order {
                            Ok(_) => (),
                            Err(err) => println!("Error: {:?}", err),
                        };
                    }
                }
                None => println!("Not found any order!"),
            },
            Err(err) => println!("Error: {:?}", err),
        };
    }

    fn print(&self) {
        println!("print order...");
        println!("\norder info:");

        match &self.order {
            Ok(result) => match result {
                Some(result) => {
                    println!("id: {:}", result.id);
                    println!("order name: {}", result.name);
                    println!("buyer: {}", result.buyer);
                    println!("credit: {}", &self.buyer.credit);
                    println!("total: {}", result.total);
                    println!("Items info:");
                    result
                        .order_items
                        .iter()
                        .for_each(|i| println!(" - {:?}", i))
                }
                None => println!("Not found any order!"),
            },
            Err(err) => println!("Error: {:?}", err),
        }
    }
}
