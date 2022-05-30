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

