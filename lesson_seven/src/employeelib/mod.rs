pub mod employee_service{
    pub struct Employee {
        pub name: String,
        pub age: i32
    }
    pub fn get_employees() -> Vec<Employee>{
        return vec![
            Employee{name: "Fuku".to_string(), age: 25}, 
            Employee{name: "Miki".to_string(), age: 22}
        ]
    }
}




