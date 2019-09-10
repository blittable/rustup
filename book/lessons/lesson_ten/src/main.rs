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
fn charge_card_test(){
    let fat_credit_card: CreditCard = CreditCard {
        card_number: "54545454".to_string(),
        available_credit: Some(100.00),
    };

    let charge_amount :Option<f32> = Some(10.0);
    let result = charge_card(&fat_credit_card, charge_amount);
    assert_eq!(result, Some(90.0));
}

#[test]
fn charge_card_test_fail(){
    let fat_credit_card: CreditCard = CreditCard {
        card_number: "54545454".to_string(),
        available_credit: Some(100.00),
    };

    let charge_amount :Option<f32> = Some(10.0);
    let result = charge_card(&fat_credit_card, charge_amount);
    assert_eq!(result, Some(80.0));
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

struct CreditCard{
    card_number: String,
    available_credit: Option<f32>,
}

fn charge_card(card: &CreditCard, charge_amount: Option<f32>) -> Option<f32> {
    match card.available_credit {
        None => None,
        Some(v) => match charge_amount {
            None => None,
            Some(a) => Some(v - a),
        }
    } 
}
