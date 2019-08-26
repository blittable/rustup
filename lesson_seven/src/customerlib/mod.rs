pub mod customer_service{
    pub struct Customer {
        pub name: String,
        pub address: String
    }
    pub fn get_top_three_customers() -> (Customer,Customer,String){
        return (
            Customer{name: "Doramon".to_string(), address: "ChiangMai, Thailand".to_string()}, 
            Customer{name: "Kitty".to_string(), address: "Tokyo, Japan".to_string()},
            "Unknow Customer".to_string()
        )
    }
}