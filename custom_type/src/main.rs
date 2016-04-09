// 注意，多了!号，表示该模块内允许出现未使用代码
#![allow(dead_code)]

use List::*;

// linked-list
enum List {
    Cons(u32, Box<List>),
    Nil
}

impl List {

    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {

        match *self {
            Cons(_, ref l) => l.len() + 1,
            Nil => 0
        }
    }

    fn stringify(&self) -> String {

        match *self {
            Cons(head, ref l) => {
                // format! 和 println! 类似，只不过它返回的是，堆内存分配字符串
                format!("{}, {}", head, l.stringify())
            },
            Nil => {
                format!("Nil")
            }
        }
    }
}

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

    // 枚举，各元素首字母必须大写
    enum Person {
        //Height = 0x111111, // 利用=，初始化赋值，只适用于纯粹的c-like enum，否则，报错
        Height,
        Weight,
        Skinny,
        Age(i32), // 非c-like enum
        Info { // 非c-like enum
            name: String
        }
    }

    let age = Person::Age(26);

    match age {
        Person::Age(i) => println!("Age is {}", i), // Age is 26
        _ => println!("nothing matched!")
    }

    let mut ls = List::new();

    ls = ls.prepend(1);
    ls = ls.prepend(2);
    ls = ls.prepend(3);

    println!("ls length is {}", ls.len()); // ls length is 3
    println!("ls stringify is {}", ls.stringify()); // ls stringify is 3, 2, 1, Nil

    // 字符串常量，默认为 &'staic str
    static LANGUAGE: &'static str = "English";
    const NUM: i32 = 24;

    println!("LANGUAGE is {}", LANGUAGE); // LANGUAGE is English
    println!("NUM is {}", NUM); // NUM is 24

}
