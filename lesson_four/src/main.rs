enum OperatorWord {
    Plus,
    Minus,
    Multiply,
    Divide
    
}

struct InputCal{
    operator: OperatorWord,
    number1: i32,
    number2: i32
}

fn main() {
    let mut op :InputCal = InputCal {
        operator: OperatorWord::Plus,
        number1: 1,
        number2: 2
    };

    println!("plus pass :{:?}", get_display_result(calculate(op)));

    op = InputCal {
    operator: OperatorWord::Divide,
    number1: 4,
    number2: 2
    };
    println!("divide pass :{:?}", get_display_result(calculate(op)));

    op = InputCal{
    operator: OperatorWord::Divide,
    number1: 4,
    number2: 0
    };
    println!("divide failed :{:?}", get_display_result(calculate(op)));
}


fn calculate(input:InputCal) -> Result<i32,String>
{
    match input{
        InputCal {operator: OperatorWord::Plus ,number1:n1 ,number2:n2} => Ok(n1 + n2),
        InputCal {operator: OperatorWord::Minus ,number1:n1 ,number2:n2} => Ok(n1 - n2),
        InputCal {operator: OperatorWord::Multiply ,number1:n1 ,number2:n2} => Ok(n1 * n2),
        InputCal {operator: OperatorWord::Divide ,number1:n1 ,number2:0} =>  Err("Division By Zero".to_string()),
        InputCal {operator: OperatorWord::Divide ,number1:n1 ,number2:n2} =>  Ok(n1 / n2)
    }
}

fn get_display_result(result:Result<i32,String>) -> String
{
    match result
    {
        Ok(n) => n.to_string(),
        Err(e) => e
    }
}

