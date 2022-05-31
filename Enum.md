# Cách khởi tạo Enum

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
## Bạn nghĩ code trên tối ưu rồi? Hơi non đấy!
