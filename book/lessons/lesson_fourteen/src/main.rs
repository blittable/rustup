///Working with iterators
///
/// An iterator has an associated type and a next method
///
///trait Iterator {
///  type Item;
///  fn next(&mut self) -> Option<Self::Item>;
///}
///
fn main() {
    standard_iteration();
    iteration_with_enumerate();
    enumerate_to_pair();
    iterate_with_into_iter();
    iter_little_things();
}

fn standard_iteration() {
    for i in 0..3 {
        println!("Standard Iterator: {:?}", i);
    }
}

///enumerate() takes an iterator and returns the value and index of the iteration
///increment the index
fn iteration_with_enumerate() {
    for (mut i, j) in (5..8).enumerate() {
        i += 1;
        println!(
            "Enumerate with increment on value - value {:?}, loop iteration: {:?}",
            i, j
        );
    }
}

///into_iter iterates the value instead of the reference
fn iterate_with_into_iter() {
    let q = vec!["Jim", "John", "Mary", "Kim", "Marcus"];

    //Standard iter() iterates over the reference &T
    for name in q.iter() {
        println!("Name is: {:?}", *name);
    }

    //into_iter() iterates over just T
    //This code will error if
    for name in q.into_iter() {
        println!("Name is: {:?}", name);
    }
}

//A good example of iterators in an idomatic rust style from tools::tidy
fn iter_little_things() {
    let semantic_version_number = "1.2.3";

    //1 - Take the elements from the split method and return a new iterator
    //2 - take that iterator and use .map to create another iterator
    //3 - create a closure that takes an iterator and invokes the next method

    let mut iterator = semantic_version_number
        .split('.')
        .map(|part| part.parse::<u32>());

    let mut part = || {
        match iterator.next() {
            None => println!("Error in referencing semantic version number.  Be sure the parts are split with '.', are integer types, and that there are no more than 3."),
            Some(e) => println!("{:?}", e), 
        };
    };

    println!("Major Version: {:?}", part());
    println!("Minor Version: {:?}", part());
    println!("Build Version: {:?}", part());
    println!("Not what you're looking for: {:?}", part());
}

