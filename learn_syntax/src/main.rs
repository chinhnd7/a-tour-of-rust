struct Point<T, U> {
    x: T,
    y: U,
}

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
    let p2 = Point {x: 3, y: 'c'};
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