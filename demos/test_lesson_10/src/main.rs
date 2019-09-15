fn main() {

    let mut our_vec = vec![1, 2, 3];

    //Uh buncho references
    crazy_update_vec(&our_vec, &our_vec, &our_vec, &our_vec, &our_vec);

    //a borrow and a mutable
    //update_vec_a(& our_vec, & mut our_vec);

    //a couple of mutables
   // update_vec_b(& mut our_vec, & mut our_vec);

    //rad simple
    //update_vec_c(& mut our_vec);


    println!("{:?}", our_vec);
    
}

fn crazy_update_vec(vec_a: &Vec<i32>, vec_b: &Vec<i32>, vec_c: &Vec<i32>, vec_d: &Vec<i32>, vec_e: &Vec<i32>) {

    //Get crazy...
    println!("{:?}", "Dogs and Cats living together...");
}

fn update_vec_a(source_vec: &Vec<i32>, target_vec: &mut Vec<i32>) {

    for i in source_vec.iter() {
        target_vec.push(i+1);
    }
}

fn update_vec_b(source_vec: &mut Vec<i32>, target_vec: &mut Vec<i32>) {

    for i in source_vec.iter() {
        target_vec.push(i+1);
    }
}

fn update_vec_c(our_vec: &mut Vec<i32>) {

    for i in our_vec.iter_mut() {
        *i = *i+1;
    }
}

