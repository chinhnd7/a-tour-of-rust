enum Coin {
    Bitcoin,
    Ethereum,
    Near,
    Solana,
}

fn get_money_in_wallet(coin: Coin) -> i32 {
    match coin {
        Coin::Bitcoin => {
            let btc: i32 = 10;
            println!("Bitcoin");
            btc
        }

        Coin::Ethereum => {
            let eth: i32 = 2;
            println!("Ethereum");
            eth
        }

        Coin::Near => {
            let near: i32 = 3;
            println!("Near");
            near
        }

        Coin::Solana => {
            let sol: i32 = 4;
            println!("Solana");
            sol
        }
    }
}

fn main() {
    let amount = get_money_in_wallet(Coin::Near);
    println!("Amount: {}", amount);
}