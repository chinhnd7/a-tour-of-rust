fn main() {
    let p2 = "string2".to_string();
    let p3 = "string3".to_string();

    let a = test1(7, &p2, &p3, 45);
    println!("result: {}", a);
}

fn test1<'a>(param1: i32, param2: &'a str, param3: &'a str, param4: i32) -> &'a str {
    if param1 == 7 && param4 > 10 {
        param2
    } else {
        param3
    }
}