# The match Control Flow

**Rust có 1 cấu trúc Control Flow rất mạnh mẽ gọi là `match`**

Chương 18 thì chúng ta sẽ học sâu về Patterns and Matching nhưng trước mắt với cấu trúc match cơ bản, mình thấy nó y hệt switch case của các ngôn ngữ khác. Nhìn này:

```rust
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
```