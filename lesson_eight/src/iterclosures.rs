#![allow(unused_variables)]

use std::str::FromStr;

fn main() {
    let names = [
             "Jim".to_string(),
             "Mariam".to_string(),
             "Collette".to_string(),
             "Mark".to_string(),
             "Tad".to_string(),
             "Tod".to_string(),
             "Chris".to_string(),
             "Susan".to_string(),
             "Christoper".to_string(),
            "Natakith".to_string(),
             "Susan".to_string(),
            "Boo".to_string(),
            "Arm".to_string(), 
            "Bob".to_string(),
       ];

    chaining(&names);
    my_chaining(&names);

    println!("{:?}", names);
}

fn chaining(names: &[String]) {
        let vector: Vec<String> = names
            .into_iter()
            .filter(|x| x.len() < 4)
            .map(|y| y.to_string())
            .collect::<Vec<String>>();
        
        println!("What's our result? : {:?}", &vector);
}

fn my_chaining(names: &[String]){
    let result = names
            .into_iter()
            .map(|x| x
                .chars()
                .rev()
                .map(|c| char::from_str(&c.to_uppercase().to_string()).unwrap())
                .collect())
            .collect::<Vec<String>>();

    println!("my result: {:?}", result);
}