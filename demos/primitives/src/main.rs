fn main() {
    println!("Some Type Stuff");

    number_types();
    arrays();
    slices();
}

fn number_types() {
    //*** numeric prefixes ****//
    let x: u8 = 255; //max value - unsigned
    let y: i8 = 127; //max value signed

    println!("And the value is: {:?}", x);

    //usize
    let max_size_lookup: usize;
    println!(
        "Pointer sized unsigned iteger type value is: {:?}",
        usize::max_value()
    );
}

fn arrays() {
    //array - fixed size!  same type of element
    let simple_array_of_ints: [i32; 5] = [2, 3, 4, 5, 1];

    //*** arrays can have heap allocated members ****//
    let lunch_team: [String; 5] = [
        "Kim".to_string(),
        "Kevin".to_string(),
        "Vee".to_string(),
        "Neng".to_string(),
        "Mate".to_string(),
    ];

    for x in lunch_team.iter() {
        println!("x is {}", x);
    }
}

//Slices do not have ownership
fn slices() {
    let lunch_team: [String; 5] = [
        "Kim".to_string(),
        "Kevin".to_string(),
        "Vee".to_string(),
        "Neng".to_string(),
        "Mate".to_string(),
    ];

    // Note the '0..3' syntax
    let slice = &lunch_team[0..3];

    for x in slice {
        println!("x is {}", x);
    }

    println!("slice is {:?}", &slice);

    // Note the '..' syntax
    let fat_slice = &lunch_team[..];

    for x in fat_slice {
        println!("Iterating fat slice: {}", x);
    }
}
