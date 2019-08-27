pub fn eat_cake() {
    println!("Eat cake !!");

    let mut v = vec!("Black Forest".to_string(), "Oreo Cheesecake".to_string(), "Carrot cake".to_string());

    for x in v.iter_mut() {
        println!("cake : {}", x);
    }
}