# Generic Types, Traits, and Lifetimes

## Generic Data Types
Có 1 ví dụ sau:

```rust
fn main() {
    let number_list = vec![20, 21, 22, 23];
    println!("The largest number is {}", get_largest(number_list));

    let char_list = vec!['a', 'b', 'x', 'y', 'z'];
    println!("The largest char is {}", get_largest_char(char_list));
}

fn get_largest(number_list: Vec<i32>) -> i32 {
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

fn get_largest_char(char_list: Vec<char>) -> char {
    let mut largest = char_list[0];

    for char in char_list {
        if char > largest {
            largest = char;
        }
    }

    largest
}
```

Ta có thể thấy sự lặp code trong ví dụ trên. Đoạn code trên chưa tối ưu ở việc tạo ra 2 function có logic giống hệt nhau chỉ để xử lý 2 kiểu dữ liệu trả về. Vậy để giải quyết vấn đề này
=> Generic Types

```rust
fn main() {
    let number_list = vec![20, 21, 22, 23];
    println!("The largest number is {}", get_largest(number_list));

    let char_list = vec!['a', 'b', 'x', 'y', 'z'];
    println!("The largest char is {}", get_largest(char_list));
}

fn get_largest<T>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    largest
}
```

Nếu ta run đoạn code trên thì sẽ nhận lỗi.
Bởi vì không phải tất cả các Types đều theo sắp xếp và có thể so sánh được bằng toán tử >

***Lỗi này có đề cập thử sử dụng trait, `std::cmp::PartialOrd`***

Fix như sau:

```rust
fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    // do logic
}
```

Phần này sẽ tìm hiểu ở phần Trait, dưới đây là các cách sử dụng generic types khác nhau.

## In Struct Definitions

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

// Đặt tên mỗi kiểu dữ liệu 1 chữ cái
// Nếu 1 chữ cái mà khi khai báo 2 kiểu dữ liệu khác nhau sẽ báo lỗi
// Nhưng 2 chữ cái khác nhau cùng biểu diễn 1 kiểu dữ liệu được
impl<T, U> Point<T, U> {
    fn mix<V, W>(self, other: Point<V, W>) -> Point<T, W>{
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point {x: 5, y : 10};
    let p2 = Point {x: "hello", y: 'c'};
    let p3 = p1.mix(p2);

    print!("{} {}", p3.x, p3.y);
}

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```