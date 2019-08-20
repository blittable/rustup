fn main() {
    let dollars = 334.33;

    //Our exchange rate changes (and needs to be mutable)
    let mut dollar_baht_exchange_rate: f32 = 31.22;

    let exchange_amount = calculate_exchange(dollars, dollar_baht_exchange_rate);
    println!("Current exchange rate: {:?}", exchange_amount);

    println!("Current exchange rate: {:?}", dollar_baht_exchange_rate);

    println!(
        "Your {:?} dollars can be exchanged for {:?} thai baht",
        dollars, exchange_amount
    );

    dollar_baht_exchange_rate = 31.11;
    println!("Updated exchange rate: {:?}", &dollar_baht_exchange_rate);
}

fn calculate_exchange(amount: f32, exchange_rate: f32) -> f32 {
    amount * exchange_rate
}
