# Error Handling

## Using a panic! Backtrace

```rust
fn main() {
    panic!("crash and burn");
}
```

***panic! chỉ in lỗi ra chứ không xử lý gì***

## Result

***Phần này thì chỉ code thôi nhé***

```rust
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// Authentication : Xác thực, bảo vệ xem bạn có được vào tòa nhà ko
// Authorization : Quyền truy cập, bạn được phép vào tầng bao nhiêu

struct Employee {
    position: Position,
    status: Status,
}

enum Position {
    IT,
    CEO,
    CTO,
    Manager,
    Marketer,
}

enum Status {
    Active,
    Denied,
}

fn try_access(employee: &Employee) -> Result<(), String> {
    match employee.status {
        Status::Denied => Err("Access denied".to_owned()),
        _ => {
            match employee.position {
                Position::CEO => Ok(()),
                Position::CTO => Ok(()),
                Position::Manager => Ok(()),
                _ => Err("Invalid position".to_owned()),
            }
        },
    }


}

fn main() {
    let manager = Employee {
        position: Position::Manager,
        status: Status::Active,
    };
    let access = try_access(&manager);
    println!("{:?}", access);

    if access.is_ok() {
        println!("access");
    }
}
```