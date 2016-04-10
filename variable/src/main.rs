fn main() {
    let mut integer = 1u32;
    let boolean: bool = true;
    let a_declare;

    {
        let integer = 123;

        a_declare = integer * 2;

        println!("current integer is {}", integer); // current integer is 123
    }

    // '_'前缀，可以让编译器不发出未使用该变量的警告
    let _unused_variable = "hello rust!";

    integer += 1;

    println!("integer is {}", integer); // integer is 2
    println!("boolean is {}", boolean); // boolean is true
    println!("a_declare is {}", a_declare); // a_declare is 246

    let boolean = "abc";

    println!("current boolean is {}", boolean); // current boolean is abc
}
