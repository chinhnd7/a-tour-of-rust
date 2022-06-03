fn main() {
    let value = Some(8);
    match value {
        Some(5) => println!("Hey 5"),
        Some(7) => println!("Hey 7"),
        _ => call_other(),
    }
}

fn call_other() {
    println!("No love for other");
}