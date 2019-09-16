pub fn print_answer(answer: &Vec<(u32, Option<u32>)>) {
    for (x, divisor) in answer {
        match divisor {
            None => {
                println!("{} is prime number.", x);
            }
            Some(v) => {
                println!("{} is not prime number because has {} as divisor.", x, v);
            }
        }
    }
}