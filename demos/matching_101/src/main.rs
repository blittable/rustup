struct MycosCoinWallet {
    number_of_coins: i32,
    exchange_bitcoin_rate: Option<f32>,
}

fn get_wallet_value_btc(mycos_wallet: MycosCoinWallet) -> Option<f32> {
    match mycos_wallet.exchange_bitcoin_rate {
        None => None,
        Some(v) => Some(v * mycos_wallet.number_of_coins as f32),
    }
}

///There a are two goals in this lesson:
///1) Introduce pattern matching syntax
///2) Introduce The Option<T> enum
fn main() {
    println!(
        "Welcome to the Mycos Currency Exchange, where unheard of Coins still incur a transaction fee."
    );

    //Lesson 4 covers other instantiation patterns
    let fat_wallet: MycosCoinWallet = MycosCoinWallet {
        number_of_coins: 23,
        exchange_bitcoin_rate: Some(2.3),
    };

    let result = get_wallet_value_btc(fat_wallet);

    match result {
        Some(i) => println!("Your wallet has the BTC value of {:?}", i),
        None => println!("There was an error in calculating your wallet size"),
    }
}
