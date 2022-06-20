# Code ví dụ về cách sử dụng Module

Thao tác syntax cơ bản:

```rust
mod front_house {
    //phải khai báo pub thì thằng khác mới gọi được (public)
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
    }

    pub mod serving {
        pub fn take_order() {}
        pub fn take_payment() {}
    }
}

fn call_order() {}

mod test {
    mod back_house {
        fn cook_order() {}
        fn fix_order() {
            // super thứ nhất để back ra cùng cấp với back_house
            // spuer thứ 2 để back ra cùng cấp với test, lúc này mới gọi được call_order()
            super::super::call_order();
            cook_order();
        }
    }
}

fn eat_at_restaurant() {
    crate::front_house::hosting::add_to_waitlist();

    // hoặc như này
    self::front_house::hosting::add_to_waitlist();

    // hoặc nếu biết nó nằm trong file này
    front_house::hosting::add_to_waitlist();
}
```

## Cách gọi các module qua các file
Xem code trong project libfiles
Và phần module crates này chủ yếu mình code để làm quen cách quản lý file, function