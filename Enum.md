# Sử dụng Enum 1 cách thật ngầu nào
## Enum with Struct
**Khởi tạo Enum và sử dụng theo ví dụ dưới đây:**
```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}
```
**Bạn nghĩ code trên tối ưu rồi? Hơi non đấy!**

## Enum with Data Types
**Đính kèm dữ liệu vào từng biến thể của Enum, vì vậy nên không cần Struct**
```rust
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

    println!("Home address is: {:#?}", home);
    println!("Loopback address is: {:#?}", loopback);

}
```

**Một sự linh động của Enum, đó là vẫn có thể lưu dạng địa chỉ V4 dưới dạng 4 values u8, còn V6 vẫn dưới dạng String. Điều này không làm được với Struct. Nhìn này, chỉ làm 1 lần thôi nhá:**

```rust
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    println!("Home address is: {:#?}", home);
    println!("Loopback address is: {:#?}", loopback);
}
```

### Cũng có thể implement method cho Enum y như Struct vậy
Here we go now:
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

## Option Enum
Trong Rust không có giá trị null. Để tránh các lỗi null gặp phải như các programming language khác, Option Enum được ra đời

```rust
enum Option<T> {
    None,
    Some(T),
}
```

**Code 1 chút xong giải thích nhé:**
```rust
    let a = Some(5);
    let str = Some("love the way you lie");
    let b: Option<i32> = None;
```

Như trên, Rust sẽ tự hiểu:
- a có kiểu dữ liệu là Option<i32>
- str có kiểu dữ liệu là Option<&str>

Nhưng với b, vì nó có giá trị None nên bắt buộc phải khai báo cho nó kiểu dữ liệu, Rust không thể hiểu None đó là None trong kiểu nào.

```rust
    let a = Some(5);
    let str = Some("love the way you lie");
    let b: Option<i32> = None;

    let test = 24;

    let sum1 = a.unwrap_or(4) + test; // Vì a có giá trị nên unwrap ra được 5
    let sum2 = b.unwrap_or(4) + test; // Vì b None nên lấy giá trị default là 4

    println!("sum1 = {}", sum1); // 29 
    println!("sum2 = {}", sum2); // 28
```

Ta luôn phải chuyển đổi Option<T> về T trước khi muốn thao tác với dữ liệu. Việc này giúp tránh các lỗi với null

**Với 1 value có thể null, luôn phải khai báo nó là Option<T> sau đó khi sử dụng value đó, luôn phải xử lý rõ ràng trường hợp giá trị null. Đây là thiết kế của Rust**
