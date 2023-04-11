struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// 構造体とメソッド:
// 矩形の構造体を定義し、面積を計算するメソッド
fn main() {
    let rect = Rectangle {
        width: 10,
        height: 20,
    };

    println!("The area of the rectangle is: {}", rect.area());
}
