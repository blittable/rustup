//Homework:
//Create a simple function, and writing a failing unit test
//You can modify and/or delete the samples here

fn main() {
    println!("Hello, world!");
}

fn higher_math() -> i32 {
    1+2
}

#[test]
fn amazing_test() {
    assert_eq!(0, 0);
}

#[test]
fn higher_math_test() {
    assert_eq!(higher_math(), 3);
}

//A bit of warning... the tests here are
//*outside* your module b/c of the ```#[cfg(test)]```
//and your module will need to be imported
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn another_amazing_test() {
        assert_eq!(higher_math(), 3);
    }
}
