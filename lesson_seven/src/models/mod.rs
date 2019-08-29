use uuid::Uuid;

// -- Order model
#[derive(Debug)]
pub struct Order {
    pub id: Uuid,
    pub name: String,
    pub buyer: String,
    pub total: f32,
    pub order_items: Vec<OrderItem>,
}

impl Order {
    pub fn new() -> Order {
        Order {
            id: Uuid::default(),
            name: String::default(),
            buyer: String::default(),
            total: 0.0,
            order_items: <Vec<OrderItem>>::new(),
        }
    }
}

// -- OrderItem model
#[derive(Debug)]
pub struct OrderItem {
    pub code: String,
    pub description: String,
    pub qty: i32,
    pub unit_price: f32,
    pub price: f32,
}

impl OrderItem {
    pub fn new() -> OrderItem {
        OrderItem {
            code: String::default(),
            description: String::default(),
            qty: 0,
            unit_price: 0.0,
            price: 0.0,
        }
    }

    pub fn new_data(data: (String, String, i32, f32)) -> OrderItem {
        OrderItem {
            code: data.0,
            description: data.1,
            qty: data.2,
            unit_price: data.3,
            price: 0.0,
        }
    }
}

// -- Buyer model
#[derive(Debug)]
pub struct Buyer {
    pub id: Uuid,
    pub name: String,
    pub credit: f32,
}

impl Buyer {
    pub fn new() -> Buyer {
        Buyer {
            id: Uuid::default(),
            name: String::default(),
            credit: 0.00
        }
    }
}