fn main() {
    // Khai báo như này sẽ không phải kiểu dữ liệu String
    // Nó chỉ là dãy các ký tự character nối tiếp nhau, và được lưu trong Stack
    let test1 = "LTMH";
    println!("{} is String?", test1);

    // Nên báo lỗi ở hàm love_flower
    // love_flower(test1, 2);

    let test2 = "LTMH".to_string();
    println!("{} is String?", test1);

    // Như này thì được
    love_flower(test2, 2);

    // Hoặc khai báo String theo cách này, lưu tại Heap
    let mut str = String::new();
    str.push_str("abc");
    println!("String is {}", str);

    let var1 = String::from("Hello");
    let var2 = var1.clone();
    println!("Get value of var1: {}", var1);

}

fn love_flower(flower: String, count: i32){
    println!("Love from {} {}", count, flower);
}
