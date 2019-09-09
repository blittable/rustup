#![allow(unused_variables)]

fn main() {
    chaining();
}

fn chaining() {

 let names: Vec<String> = vec![
                "Jim",
                "Mariam",
                "Collette",
                "Mark",
                "Hong",
                "Tad",
                "Tod",
                "Chris",
                "Susan",
                "Christoper",
                "Natakith",
                "Por",
                "Susan",
                "Pop",
                "Boo",
                "Arm", 
                "Bob",
                "Yam"
            ]
            .into_iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>();
        
        /*let vector: Vec<String> = names
            .into_iter()
            .filter(|x| x.len() < 4)
            .map(|y| y.to_string())
            .collect::<Vec<String>>();
        
        println!("What's our result? : {:?}", &vector);*/

        let who_make_me = |feeling: &String, names: Vec<String>| -> Vec<String> { 
            let mut whos: Vec<String> = Vec::new();
            feeling
            .chars()
            .for_each(|c| whos.push(names.clone().into_iter().find(|n| n.starts_with(c)).unwrap()));
            
            return whos;
        };

        let feeling: String = String::from("HAPPY");
        println!("Friends who make me {} : {:?}", 
                &feeling, 
                who_make_me(&feeling, names)
                    .into_iter()
                    .map(|n| format!("({}){}", &n[..1], &n[1..].to_string()))
                    .collect::<Vec<String>>());
}