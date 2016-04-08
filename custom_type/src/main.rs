fn main() {
    // struct, enum
    // const, static

    struct Nil;
    struct Pair(i32, f64);
    struct Point {
        x: f64,
        y: f64
    }

    // #[allow(dead_code)]，允许出现未使用的变量
    #[allow(dead_code)]
    struct Rectangle {
        p1: Point,
        p2: Point
    }

    let point: Point = Point {x: 2.2, y: 4.4};

    println!("point => ({}, {})", point.x, point.y); // point => (2.2, 4.4)

    let Point {x: my_x, y: my_y} = point;

    println!("my_x: {}", my_x); // my_x: 2.2
    println!("my_y: {}", my_y); // my_y: 4.4

    // 未使用的变量，可以增加下划线前缀，以免编译器发出警告
    let _nil = Nil;

    let _rectangle = Rectangle {
        p1: Point {x: 1.1, y: 3.3},
        p2: point
    };

    let pair = Pair(1, 32.1);
    let Pair(integer, decimal) = pair;

    println!("pair => ({}, {})", integer, decimal); // pair => (1, 32.1)

}
