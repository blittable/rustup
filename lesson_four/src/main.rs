//Write some code that does the following:
//1: uses an Option<T>
//2: uses Result<T, E> type
//3: uses 'match'
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

fn main() {
    let fat_credit_card: CreditCard = CreditCard {
        card_number: "54545454".to_string(),
        available_credit: Some(100.00),
    };

    let empty_credit_card: CreditCard = CreditCard {
        card_number: "44444444".to_string(),
        available_credit: None,
    };
    let charge_amount :Option<f32> = Some(10.0);
    let result = charge_card(&fat_credit_card, charge_amount);

    match result {
        Some(i) => println!("Your card number {:?} was charged for {:?} and balance is {:?}",fat_credit_card.card_number, charge_amount, i),
        None => println!("Your card  number {:?} has problem.",fat_credit_card.card_number),
    }

    let charge_amount2 :Option<f32> = Some(10.0);
    let result2 = charge_card(&empty_credit_card, charge_amount2);

    match result2 {
        Some(i) => println!("Your card number {:?} was charged for {:?} and balance is {:?}",empty_credit_card.card_number, charge_amount2, i),
        None => println!("Your card  number {:?} has problem.",empty_credit_card.card_number),
    }
}
