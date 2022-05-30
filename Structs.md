# Các thao tác cơ bản khởi tạo, copy value, thay đổi value trong Structs

**Code ví dụ**

```rust
#[derive(Debug)]

struct Member {
    username: String,
    email: String,
    age: u64,
    active: bool,
}

fn main() {
    let mut member1: Member = Member{
        email: String::from("junnguyen@gmail.com"),
        username: String::from("junkyu"),
        age: 28,
        active: true,
    };

    // Thay đổi value của member1
    member1.username = String::from("ltmh");
    println!("member1 = {:#?}", member1);

    let member2: Member = create_new_member(String::from("Jun Nguyen"),
     String::from("junnguyen@gmail.com"),
     25);
     println!("member2 = {:#?}", member2);

     let member3: Member = Member {
         username: String::from("post malone"),
         // Copy value từ các fields còn lại của member2
         ..member2
     };

     println!("member3 = {:#?}", member3);
}

fn create_new_member(username: String, email: String, age: u64) -> Member {
    Member {
        email: email,
        username: username,
        age: age,
        active: true,
    }
}
```

## Example Program Using Structs

**Đây là cách các giang hồ thật sự dùng Struct**
```rust
#[derive(Debug)]

struct Rectangle {
    length: i32,
    width: i32,
}

impl Rectangle {
    fn calArea(&self) -> i32 {
        self.length * self.width
    }

    fn isBigger(&self, otherRectangle: &Rectangle) -> bool {
        self.calArea() > otherRectangle.calArea()
    }

    // Associated Functions
    // Đây là function không cần dùng đến instance
    // Khi gọi nó chúng ta dùng ::  Trong trường hợp này là:
    // Rectangle::createSquare(60)
    fn createSquare(length: i32) -> Rectangle {
        Rectangle {
            length: length,
            width: length,
        }
    } 
}

fn main() {
    let hcn1: Rectangle = Rectangle{
        length: 50,
        width: 30,
    };

    let hcn2: Rectangle = Rectangle{
        length: 40,
        width: 40,
    };

    println!("Diện tích HCN1 là: {}", hcn1.calArea());
    println!("HCN1 lớn hơn HCN2: {}", hcn1.isBigger(&hcn2));

    let hcn3: Rectangle = Rectangle::createSquare(60);

    println!("hcn3 = {:#?}", hcn3);
}
```

**Chúng ta hoàn toàn có thể tách 3 method trong 1 block implement thành 3 block:**
```rust
impl Rectangle {
}
```
**khác nhau**

Mỗi struct cho phép có Multiple impl Blocks.
Về việc tách và quản lý block này sẽ đề cập ở Chapter 10 :) hihi