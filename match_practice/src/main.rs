#[derive(Debug)]
enum Type {
    VeryOld,
    Old,
    New,
}

enum Coin{
    One,
    Ten,
    Hundred,
    FiveHundred(Type),
    Thousand,
}

fn value_coins(coin: Coin) -> u32{
    match coin {
        Coin::One => 1,
        Coin::Ten => 10,
        Coin::Hundred => 100,
        Coin::FiveHundred(coin_type) => {
            println!("{:?}Type 500Yen Coin!!", coin_type); 
            500
        }
        Coin::Thousand => 1000,
    }
}

fn main() {
    let coin_type = Type::New;
    let my_coin = Coin::FiveHundred(coin_type);
    println!("{}",value_coins(my_coin));
}
