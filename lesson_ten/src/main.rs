//Homework:
//Create a simple function, and writing a failing unit test
//You can modify and/or delete the samples here

mod StringHelper;

fn main() {
    //println!("Hello, world!")
    let result = StringHelper::to_pascal_str("helloword ! ");
    println!("{}", result);
}

fn higher_math() -> i32 {
    1 + 2
}

#[test]
fn amazing_test() {
    assert_eq!(0, 0);
}

#[test]
fn higher_math_test() {
    assert_eq!(higher_math(), 3);
}

// My unit tests
#[test]
fn to_pascal_str_test() {
    let expect = "HelloWord";
    let result = StringHelper::to_pascal_str("hello word");
    assert_eq!(expect, result);
}

#[test]
fn to_pascal_st_long_string_test() {
    let expect = "HelloWordAbcZzzz";
    let result = StringHelper::to_pascal_str("hello word Abc    zZZz");
    assert_eq!(expect, result);
}

#[test]
fn to_pascal_str_empty_test() {
    let expect = "";
    let result = StringHelper::to_pascal_str("");
    assert_eq!(expect, result);
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
