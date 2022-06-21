# Vector
## Storing Lists of Values with Vectors

***Thực sự các file md từ nay về sau sẽ được cập nhật nội dung khi mình ôn lại sau, hiện tại mình chỉ code và comment để đẩy nhanh tiến độ***

Thao tác với Vector:

```rust
fn main() {
    let a = [1, 2, 3];

    // khai báo vector cách 1
    let v = vec![1, 2, 3, 4, 5, 6];

    // khai báo cách 2
    let mut v2 = Vec::new();
    v2.push(1);
    v2.push(2);
    v2.push(3);
    v2.push(4);

    println!("{:?}", v2);

    // Kiểm tra số level của game, vì 1 game có thể liên tục update level mới,
    // nên ta dùng vector, nếu có level trong v thì sẽ hiển thị level
    match v.get(4) {
        Some(level) => println!("This level is {}", level),
        None => println!("None level"),
    }
}
```

```rust
    let mut v = vec![1, 2, 3, 4, 5, 6];

    // lấy các giá trị trong vector
    for i in &mut v {

        // Toán tử *, để get value của i
        // Trình bày ở chapter 15:
        // Following the Pointer to the Value with the Dereference Operator
        *i += 10;
    }

    for i in &mut v {
        println!("{}", i);
    }

    // result:
    // 11 12 13 14 15 16
```

***Cái ví dụ này người ta dùng với vector rất là ít, mà người ta dùng với Enum rất nhiều như này***

## Using an Enum to Store Multiple Types

```rust
fn main() {
    
    enum SheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let mut row = vec![
        SheetCell::Int(5),
        SheetCell::Float(27.02),
        SheetCell::Text(String::from("Blue")),        
    ];

    match &mut row[1] {
        &mut SheetCell::Float(mut i) => {
            i += 2.0;
            println!("{}", i);
        },
        _ => println!("This is not a float"),
    }

    // result: 29.02
}
```

# String
## Creating a New String

```rust
fn main() {
    // String trong Rust hỗ trợ UTF-8, tiếng Việt, tiếng Ấn... xịn

    // Các cách khởi tạo String
    let s1 = String::new();
    let s2 = String::from("This is String");
    let s3 = "A String".to_string();

    // Lưu ý khi nối String
    // gặp lỗi, không cho cộng 2 String
    // help: nên cộng 1 chuỗi và tham chiếu 1 chuỗi
    let s4 = s2 + " hehehe " + &s3;
    println!("{}", s4);

    // lúc này s2 đã bị sở hữu
}
```

# HashMap

## Creating a New Hash Map
```rust
use std::collections::HashMap;

fn main() {
    let mu = String::from("MU");
    let mc = String::from("MC");

    let mut scores = HashMap::new();
    scores.insert(mu, 10);
    scores.insert(mc, 9);

    println!("{:?}", scores);

    let team_name = String::from("MU");
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{} {}", key, value);
    }
    println!("score = {:?}", score);

    // result:
    // {"MC": 9, "MU": 10}
    // MC 9
    // MU 10
    // score = Some(10)
}
```

## Overwriting a Value

Sử dụng insert

```rust
    let mut scores = HashMap::new();
    scores.insert(String::from("MU"), 10);
    scores.insert(String::from("MU"), 20);

    println!("scores = {:?}", scores);

    // result:
    // scores = {"MU": 20}
```

## Only Inserting a Value If the Key Has No Value

Sử dụng entry:
Nếu có giá trị rồi ko insert, còn nếu chưa thì insert data 

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.entry(String::from("MU")).or_insert(10);
    scores.entry(String::from("MC")).or_insert(20);
    scores.entry(String::from("MU")).or_insert(30);


    println!("scores = {:?}", scores);

    // result:
    // scores = {"MU": 10, "MC": 20}
}
```

## Updating a Value Based on the Old Value

Ứng dụng cho việc đếm từ:

```rust
use std::collections::HashMap;

fn main() {
    let text = "hello world this is wonderful world";

    let mut map = HashMap::new();
    for i in text.split_whitespace() {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map)

    
    // result:
    // {"is": 1, "this": 1, "hello": 1, "wonderful": 1, "world": 2}
}
```