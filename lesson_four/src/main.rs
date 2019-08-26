fn convert_with_default(str_val: String) -> Option<bool> {
    let mut result: bool = false;
    
    if str_val != "1".to_string() && str_val != "0".to_string() {
        None
    } 
    else {
        if str_val == "1".to_string() {
            result = true;
        }
        else if str_val == "0".to_string() {
            result = false;
        }

        Some(result)
    } 
}

fn convert_with_error(str_val: String) -> Result<bool, String> {
    let mut result: bool = false; 

    if str_val != "1".to_string() && str_val != "0".to_string() {
        Err("Invalid boolean value.".to_string())
    } 
    else {
        if str_val == "1".to_string() {
            result = true;
        }
        else if str_val == "0".to_string() {
            result = false;
        }

        Ok(result)
    }
}

fn main() {
    match convert_with_default("1".to_string()) {
        None => println!("No value returned"),
        Some(b) => println!("Boolean value is: {:?}", b)
    }

    match convert_with_default("1234".to_string()) {
        None => println!("No value returned"),
        Some(b) => println!("Boolean value is: {:?}", b)
    }

    match convert_with_error("0".to_string()) {
        Ok(b) => println!("Boolean value is: {:?}", b),
        Err(err) => println!("{:?}", err)
    }

    match convert_with_error("1234".to_string()) {
        Ok(b) => println!("Boolean value is: {:?}", b),
        Err(err) => println!("{:?}", err)
    }
}