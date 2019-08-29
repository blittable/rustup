use crate::services::sale::OrderService;
use crate::models::*;

pub trait OrderServiceTrait {
    fn new() -> OrderService;
    fn load(&mut self, order_name: String, buyer: Buyer);
    fn add_item(&mut self, item: OrderItem);
    fn calculate(&mut self);
    fn print(&self);
}