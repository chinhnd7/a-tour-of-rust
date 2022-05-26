fn main() {
    let mut s1 = String::from("Hello");
    let len = calculate_lengjht(&mut s1);

    println!("Do dai cua {} la {}", s1, len);
}

fn calculate_lengjht(some_string: &mut String) -> usize {
    some_string.push_str(" world!");
    let length = some_string.len();
    length
}

