
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    let a = scores.entry(String::from("MU")).or_insert(10);
    let b = scores.entry(String::from("MC")).or_insert(20);
    let c = scores.entry(String::from("MU")).or_insert(30);


    println!("scores = {:?}", scores);
    println!("a = {} b = {} c = {}", a, b, c);

    // result:
    // scores = {"MU": 10, "MC": 20}
}
