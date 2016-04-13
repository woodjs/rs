#![allow(unreachable_code)]

fn main() {

    //! if/else
    //! loop
    //! while
    //! for in
    //! match 绑定：n @ 1 ... 10
    //! if let
    //! while let
    //! 取值：*， 解构赋值：&, ref, ref mut

    // if/else，各分支的返回值，必须为相同的类型
    let x = 1;
    let a =
        if x > 10 {
            11
        } else if x < 10 && x > 1 {
            22
        } else if x == 1 {
            33
        } else {
            44
        };

    println!("a is {}", a); // a is 33

    let mut count = 0u32;

    'outer: loop {

        count += 1;

        println!("{}", count);

        'inner: loop {

            if count == 1 {
                println!("current count is {}", count);
                continue 'outer;
            }

            if count == 2 {
                println!("end count is {}", count);
                break 'outer;
            }

        }

        // 1
        // current count is 1
        // 2
        // end count is 2
    }

    let mut n = 5u32;

    while n > 0 {
        n -= 1;
        println!("current n is {}", n);

        // current n is 4
        // current n is 3
        // current n is 2
        // current n is 1
        // current n is 0
    }

    // 不包括3
    for n in 1..3 {
        println!("current n is {}", n);

        // current n is 1
        // current n is 2
    }

    let number = 11;

    match number {
        // 包括3
        1...3 => println!("branch 1 matched"),
        5 | 8 | 9 => println!("branch 2 matched"),
        n @ 10 ... 19 => println!("branch 3 matched，n is {}", n),
        _ => println!("no branch matched")

        // branch 3 matched，n is 11
    }

    #[allow(dead_code)]
    enum Pair {
        Holder,
        Point(u32, u32)
    }

    let pair = Pair::Point(1, 2);

    match pair {
        Pair::Point(1, x) if x == 2 => println!("x is {}", x),
        Pair::Point(y, 2) => println!("y is {}", y),
        _ => println!("don't bind the value to a variable")

        // x is 2
    }

    #[derive(Debug)]
    struct Foo {
        x: (u32, u32),
        y: u32
    }

    let foo = Foo {
        x: (1, 2),
        y: 1
    };

    let Foo {x: (a, b), ..} = foo;
    println!("a is {}, b is {}", a, b); // a is 1, b is 2

    let Foo {x, ..} = foo;
    println!("x is {:?}", x); // x is (1, 2)

    let reference = &3;
    println!("reference is {}", reference); // reference is 3
    println!("reference is {}", *reference); // reference is 3

    let ref r = 4;
    println!("r is {}", r); // r is 4
    println!("r is {}", *r); // r is 4

    let w = 5;
    println!("w is {}", w); // w is 5
    //println!("w is {}", *w); // 报错

    // Option<i32>，是一个特殊的类型，值必须为None
    let a: Option<i32> = None;
    let b = Some(8); //Some，是一个特殊的类型

    if let Some(i) = b {
        println!("i is {}", i); // i is 8
    } else {
        println!("don't match a number");
    }

    let mut a = Some(2);

    while let Some(i) = a {

        if i == 2 {
            println!("current i is {}", i);
            break;
        }

        println!("i is {}", i);

        a = Some(i - 1);

        // i is 3
        // current i is 2
    }

}
