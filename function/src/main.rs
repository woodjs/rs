fn main() {

    //! method，用impl关键字定义
    //! closure，在rust中，又称为lambda表达式，参数用||包围，参数和返回值类型可推断出
    //! Fn(&T), FnMut(&mut T), FnOnce(T)
    //! 当函数与闭包同名时，闭包覆盖同名函数
    //! 同名函数，编译报错
    //! let定义同名变量，外部表现同解释性语言如javascript一致
    //! 目前rust只支持函数或闭包返回确定的类型，想要将闭包作为参数返回，需要使用move关键字和Box，如Box<Fn()>
    //! move关键字，定义所有的capture以值的形式存在

    use std::mem;

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

    fn func(i: i32) -> i32 { i + 1}

    let closure_annotated = |i: i32| -> i32 { i + 1 };

    let i = 1;

    println!("func: {}", func(i)); // func: 2
    println!("closure_annotated: {}", closure_annotated(i)); // closure_annotated: 2

    let movable = Box::new(3);

    let consume = || {
        println!("movable: {}", movable);
        mem::drop(movable);
    };

    consume(); // movable: 3

    // 约束只能使用Fn, FnMut或FnOnce
    fn apply<F>(f: F) -> Box<Fn()> where F: Fn(i32) -> i32 {
        f(5);
        let text = "this is a value import from closure!";
        Box::new(move || println!("{}", text))
    }

    fn test1(x: i32) -> i32 {
        println!("function test1 used as arguments, print {}!", x);
        0
    }

    let test = |x| {
        let mut line = "hello".to_owned();

        line.push_str(" number");

        println!("this line will print {} {}!", line, x);

        mem::drop(line);
        0
    };

    test(1); // this line will print hello number 1!
    let a = apply(test); // this line will print hello number 5!

    test1(2); // function test1 used as arguments, print 2!
    let b = apply(test1); // function test1 used as arguments, print 5!

    a(); // this is a value import from closure!
    b(); // this is a value import from closure!

    let a = 1;
    println!("{}", a); // 1
    let a = 2;
    println!("{}", a); // 2
}
