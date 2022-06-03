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
    Bitcoin(Balance),
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

## Catch-all Patterns and the _ Placeholder

Bắt hết các case còn lại: `other` 

```rust
fn main() {
    let value = Some(8);
    match value {
        Some(5) => println!("Hey 5"),
        Some(7) => println!("Hey 7"),
        other => call_other(other),
    }
}

fn call_other(some_num: Option<u8>) {
    println!("No love for other {:#?}", some_num);
}
```

***Sử dụng `other` khi chúng tan cần chính giá trị này để thực hiện các hành động tiếp theo.***

### Nếu không quan tâm đến các giá trị còn lại `_`

Dùng _ Placeholder
```rust
fn main() {
    let value = Some(8);
    match value {
        Some(5) => println!("Hey 5"),
        Some(7) => println!("Hey 7"),
        _ => call_other(),
    }
}

fn call_other() {
    println!("No love for other");
}
```

### Nếu không quan tâm đến các giá trị còn lại + không thực hiện hành động gì

```rust
fn main() {
    let value = Some(8);
    match value {
        Some(5) => println!("Hey 5"),
        Some(7) => println!("Hey 7"),
        _ => (), // here
    }
}
```

## If let

**Với match khi ta không quan tâm đến value so sánh, và cũng không xử lý gì, như này này `_ => (),`. Vậy thì ta nên sử dụng cách viết với `if let`**
Here we go:

```rust
let value = Some(117);
if let Some(bth_day) = value {
    println!("Jun's birthday is {}", bth_day);
}
```

**if let and else**

Well, khi mà sử dụng `match`, tại mỗi block đối sánh mà logic của nó quá dài dòng, ta hoàn toàn có thể cân nhắc để sử dụng `if let and else`

```rust
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
```

```rust
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
```