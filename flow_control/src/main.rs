#![allow(unreachable_code)]

fn main() {

    //! if/else
    //! loop
    //! while
    //! for in
    //! match
    //! if let
    //! while let

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

    let number = 9;

    match number {
        // 包括3
        1...3 => println!("branch 1 matched"),
        5 | 8 | 9 => println!("branch 2 matched"),
        _ => println!("no branch matched")

        // branch 2 matched
    }
}
