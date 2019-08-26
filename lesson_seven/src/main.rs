pub mod employeelib;
use employeelib::employee_service::{get_employees};

pub mod customerlib;
use customerlib::customer_service::{get_top_three_customers};

fn main() {
    println!("** Get Employee List **");
    let employees = get_employees();
    for emp in employees.iter() {
            println!("Name: {:?}, Age: {:?}", emp.name, emp.age);
    }

    println!("** Get Customer List **");
    let (first_customer, second_customer, unknow) = get_top_three_customers();
    println!("Name: {:?} Address: {:?}", first_customer.name, first_customer.address);
    println!("Name: {:?} Address: {:?}", second_customer.name, second_customer.address);
    println!("{:?}", unknow);
    
    // println!("Hello, world!");

    // let a = [1, 2, 3];

    // a.iter().map(|&x| x * 2);

    // assert_eq!(vec![2, 4, 6], doubled);
}
