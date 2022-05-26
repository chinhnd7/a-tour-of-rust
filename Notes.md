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

### Ownership rules áp dụng cho các kiểu dữ liệu lưu trong Heap, còn Stack thì không

***Vì mới học nên trước mắt chúng ta sẽ lấy ví dụ với data structure phổ biến: String***
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

## Ownership với Function
    Việc truyền giá trị cho function, cũng cần lưu ý về Ownership. Xem ví dụ dưới đây:
```rust
fn main() {
    let jun = String::from("my heart");

    // Lúc này variable "jun" đã không còn là owner của value "my heart" nữa.
    takes_ownership(jun);
    // Báo lỗi borrow of moved value: `jun`
    println!("She takes {}", jun);

    // Vì khi truyền vào hàm, "jun" đã bị drop
}

fn takes_ownership(jun: String) {
    println!("She takes {}", jun);
}
```

# Reference Tham chiếu
    Vậy khi dùng 1 variable truyền vào hàm khác rồi, ta muốn dùng lại variable đó thì trong hàm đó lại phải trả lại đúng không? Như này này:
```rust
fn main() {
    let jun = String::from("my heart");
    // Hàm calculate đã trả về 1 variable mới, để nhận lại giá trị của "jun"
    let (str, len) = calculate_lenght(jun);

    println!("Do dai cua {} la {}", str, len);
}

fn calculate_lenght(jun: String) -> (String, usize){
    let length = jun.len();
    (jun, length)
}
```

    Vậy là việc này rất mất thời gian, và rườm rà đúng không?
    Có cách nào khác không?

    **Truyền tham chiếu, như sau:**
```rust
fn main() {
    let s1 = String::from("Hello");
    let len = calculate_lengjht(&s1);

    println!("Do dai cua {} la {}", s1, len);
}

fn calculate_lengjht(some_string: &String) -> usize {
    let length = some_string.len();
    length
}
```
    **Vậy muốn thay đổi luôn giá trị của s1 thì sao?**
```rust
fn main() {
    // thêm mut
    let mut s1 = String::from("Hello");
    // mut again :))
    let len = calculate_lengjht(&mut s1);

    println!("Do dai cua {} la {}", s1, len);
}

// and again :))))
fn calculate_lengjht(some_string: &mut String) -> usize {
    some_string.push_str(" world!");
    let length = some_string.len();
    length
}
```