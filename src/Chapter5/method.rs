#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // メソッド実践編(自分で書いたからドキュメントとは異なる)
    fn can_hold(&self, rect: &Rectangle) -> bool {
        if self.height >= rect.height && self.width >= rect.width {
            true
        } else {
            false
        }
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    // rect1はそれぞれのrectを中に入れることができるか？
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect3));
}