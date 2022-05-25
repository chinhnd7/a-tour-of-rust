# Ownership, Borrowing & Reference

## Stack, Heap & các kiểu khởi tạo String

```rust
fn main() {
    // Khai báo như này sẽ không phải kiểu dữ liệu String
    // Nó chỉ là dãy các ký tự character nối tiếp nhau, và được lưu trong Stack
    let test1 = "LTMH";
    println!("{} is String?", test1);

    // Nên báo lỗi ở hàm love_flower()
    love_flower(test1, 2);

    let test2 = "LTMH".to_string();
    println!("{} is String?", test1);

    // Như này thì được
    love_flower(test2, 2);

    // Hoặc khai báo String theo cách này, lưu tại Heap
    let mut str = String::new();
    str.push_str("abc");
    println!("String is {}", str);
}

fn love_flower(flower: String, count: i32){
    println!("Love from {} {}", count, flower);
}
```

## Ownership Rules
    ** 1. Mỗi 1 giá trị chỉ có 1 biến được gọi là chủ sở hữu (one value one variable (owner))
        Each value in Rust has a variable that’s called its owner.

    2. Chỉ có 1 owner tại 1 thời điểm (1 biến không thể có 2 owner tại 1 thời điểm)
        There can only be one owner at a time.

    3. Khi owner đi ra khỏi phạm vi hoạt động, value sẽ bị drop.
        When the owner goes out of scope, the value will be dropped.**

```rust
    let var1 = String::from("Hello");
    let var2 = var1;
    println!("Get value of var1: ", var1);
```

    Báo lỗi *** borrow of moved value: `var1`***

    Lúc này var2 đã là owner của giá trị "hello". var1 đã bị drop.
    
    Có thể copy giá trị var1 cho var2 bằng cách này:

```rust
    let var1 = String::from("Hello");
    let var2 = var1.clone();
    println!("Get value of var1: ", var1);
```