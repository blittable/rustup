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
        
        let vector: Vec<String> = names
            .into_iter()
            .filter(|x| x.len() < 4)
            .map(|y| y.to_string())
            .collect::<Vec<String>>();
        
        let longest_name = &names
        .into_iter().max_by(|x, y| x.len().cmp(&y.len()));

        let longest_position = &names
        .into_iter().position(|x| x.to_string() == longest_name.unwrap().to_string());


        println!("What's our result? : {:?}", &vector);
        println!("The longest name is : {:?}", &longest_name.unwrap());
        println!("Index of the longest name is : {:?}", &longest_position.unwrap());
}