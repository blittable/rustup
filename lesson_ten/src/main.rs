//Homework:
//Create a simple function, and writing a failing unit test
//You can modify and/or delete the samples here

fn main() {
    println!("Hello, world!");
}

fn get_longest_string(input:&[String]) -> String{

    let longest_string = &input
        .into_iter().max_by(|x, y| x.len().cmp(&y.len()));
    return longest_string.unwrap().to_string();
}

#[test]
fn test_pass() {
     let names = [
             "Mate".to_string(),
             "etc3tera".to_string(),
             "iamkhwan".to_string(),
             "john".to_string(),
             "kanoon".to_string(),
             "toomcutlerz".to_string(),
             "kevinnel".to_string()
       ];
    assert_eq!("toomcutlerz".to_string(), get_longest_string(&names));
}

#[test]
fn test_failed() {
     let names = [
             "Mate".to_string(),
             "etc3tera".to_string(),
             "iamkhwan".to_string(),
             "john".to_string(),
             "kanoon".to_string(),
             "kevinnel".to_string()
       ];
    assert_eq!("toomcutlerz".to_string(), get_longest_string(&names));
}

