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

        let num_of_people :&usize =  &vector.clone().into_iter()
            .count();

        let total_length_of_names :&usize =  &names.clone()
            .into_iter()
            .map(|t| t.chars().count())
            .sum();
        
        println!("What's our result? : {:?}", &vector);
        println!("Number of people : {:?}", &num_of_people);
        println!("Total length of names : {:?}", &total_length_of_names);

        
}
