fn main() {

    // method，用impl关键字定义
    struct Point {
        x: f32,
        y: f32
    }

    #[allow(dead_code)]
    impl Point {

        fn origin() -> Point {
            Point {x: 1.0, y: 2.0}
        }

        fn new(x: f32, y: f32) -> Point {
            Point {x: x, y: y}
        }

        fn get_x(&self) -> f32 {
            self.x
        }

        fn get_y(&self) -> f32 {
            self.y
        }

    }

    // method没有&self或self参数，通过::调用
    let p = Point::origin();

    let x = p.get_x();
    let y = p.get_y();

    println!("p.x is {}", x); // p.x is 1
    println!("p.y is {}", y); // p.y is 2

}
