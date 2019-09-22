fn main() {
    the_real_owner();
    test_to_owned_and_clone();
}

//Borrow test One shows borrowing != taking ownership
// 1) A borrow created in one scope
// 2) An assignment of a heap allocated string in another scope to the borrowed variable in the inner scope 
// 3) A reference to the borrowed value after the assigned value goes out of scope 
fn the_real_owner() {

    // WHO'S THE REAL OWNER 
    let mut name_borrower = &String::default();
    {
        let name: String = String::from("Mycos");
        name_borrower = &name;
    }

    //The real owner เขาตายแล้ว 
    //println!("And the Winner is: {:?} ", &name_borrower);
}


/// https://doc.rust-lang.org/std/borrow/trait.ToOwned.html


fn test_to_owned_and_clone() {
    let s1 = "I am boxed and owned".to_string();
    let s2 = "I am boxed and owned".to_string();

    let c1 = s1.to_owned();
    let c2 = s2.clone();

    println!("{:?}", c1);
    println!("{:?}", c2);

    println!("{:?}", c1 == s1);   // compile-time error here (see below)
    println!("{:?}", c2 == s2);
}
