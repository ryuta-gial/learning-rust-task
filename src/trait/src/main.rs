// トレイト:
// トレイトを使用して、異なる図形（矩形、円）の面積を計算
// 矩形の面積は50.0、円の面積は約28.27
trait Area {
    fn area(&self) -> f64;
}

struct Rectangle {
    width: f64,
    height: f64,
}

struct Circle {
    radius: f64,
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

fn calculate_area<T: Area>(shape: &T) -> f64 {
    shape.area()
}

fn main() {
    let rectangle = Rectangle {
        width: 5.0,
        height: 10.0,
    };
    let circle = Circle {
        radius: 3.0,
    };

    let rectangle_area = calculate_area(&rectangle);
    let circle_area = calculate_area(&circle);

    println!("The area of the rectangle is: {:.2}", rectangle_area);
    println!("The area of the circle is: {:.2}", circle_area);
}
