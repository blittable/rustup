fn main() {
    //Move
    let name1: String = "AONG".to_string();
    let name2: String = name1;
    println!("Moved name {}", name2); //Cannot use name1 because it gone (it moved to name2)

    //Borrow
    let s1: String = "MYCOS".to_string();
    let s2: &String = &s1;
    println!("Onwer is s1 {}. Borrower is s2 {}", s1, s2); //Can use both variable

    //Change cross function
    let mut name: String = "MYCOS".to_string();
    println!("Before change {}", name);
    full_name(&mut name); //Change value vie borrow mutable
    println!("After change {}", name);
}

fn full_name(name: &mut String) {
    name.push_str(" Technology");
}