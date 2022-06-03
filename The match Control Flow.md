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

**Nếu từ mỗi ví Bitcoin ta muốn chỉ ra chủ sở hữu thuộc loại Cá nào, ta cần thêm 1 value lưu bên trong biến thể Enum Bitcoin này**
```rust
enum Balance {
    Small,
    Intermedium,
    Fish,
    Shark,
}

enum Coin {
    // same
}

fn get_money_in_wallet(coin: Coin) -> i32 {
    match coin {
        Coin::Bitcoin(balance) => {
            let btc: i32 = 10;
            println!("Bitcoin");
            println!("I am a {:#?}", balance);
            btc
        }
        // same
    }
}

fn main() {
    let amount = get_money_in_wallet(Coin::Bitcoin(Balance::Shark)); // I am a Shark
    println!("Amount: {}", amount);
}
```

## Matching with Option<T>

**Việc xử lý matching với Option<T>, yah u know, điều này là cần thiết khi xử lý các value null**
```rust
fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("Six: {:#?}", six);
    println!("None: {:#?}", none);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(x) => Some(x + 1),
        // None => None,
    }
}
```

Nếu không match trường hợp None, chúng ta sẽ gặp lỗi ở đây:
`error[E0004]: non-exhaustive patterns: `None` not covered`

Kết quả sau khi bỏ comment và cover None case, and run:

```rust
Six: Some(
    6,
)
None: None
```