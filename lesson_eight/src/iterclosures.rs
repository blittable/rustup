#![allow(unused_variables)]

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
        
    let add_desciption = |name: &str| "Name is ".to_string() + name;
    
    let mut vector: Vec<String> = names
        .into_iter()
        .filter(|x| x.len() < 4)
        .map(|y| -> String { add_desciption(&y).to_string()}) //This code mean 'use two closures in your solution' or not?
        .collect::<Vec<String>>();
    
    vector.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    println!("What's our result? : {:?}", &vector);

}