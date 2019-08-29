#![allow(unused_variables)]

use std::collections::HashSet;

fn main() {
    chaining();
}

fn chaining() {
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

    let vector: Vec<String> = names
        .into_iter()
        .filter(|x| x.len() < 4)
        .map(|y| y.to_string())
        .collect::<Vec<String>>();

    // Select first and where like
    let vector2 = names
        .into_iter()
        .filter(|x| x.to_lowercase().contains("bo"))
        .map(|y| y.to_string())
        .next();

    // Select and where in
    let vector3 = names
        .into_iter()
        .filter(|x| ["Collette".to_string(), "Susan".to_string()].contains(x))
        .map(|y| y.to_string())
        .collect::<Vec<String>>();

    // Select distinct and tranform
    let vector4 = names
        .into_iter()
        .filter(|x| x.to_lowercase().starts_with("c"))
        .map(|y| y.to_string())
        .collect::<HashSet<_>>()
        .into_iter()
        .map(|i| i.chars().map(|c| format!("{} - ", &c)).collect())
        .map(|j: String| format!("Hi, {}", &j[..j.len() - 2].trim()))
        .collect::<Vec<String>>();

    println!("What's our result? : {:?}", &vector);
    println!("What's our result2? : {:?}", &vector2);
    println!("What's our result3? : {:?}", &vector3);
    println!("What's our result4? : {:?}", &vector4);
}
