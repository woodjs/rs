// 免去cast时编译器溢出警告
#![allow(overflowing_literals)]

fn main() {

    //! as
    //! type，定义类型

    let decimal1 = 12.333_f32;
    let decimal2 = decimal1 as f64;
    let integer1 = 8;
    let integer2 = integer1 as u16;

    println!("decimal1 is {}", decimal1); // decimal is 12.333
    println!("decimal2 is {}", decimal2); // decimal2 is 12.333000183105469

    println!("integer1 is {}", integer1); // integer1 is 8
    println!("integer2 is {}", integer2); // integer1 is 8

    let size1 = 1isize;
    let size2 = 1usize;

    // &，不能去掉，否则报错
    println!("size1 is {}", std::mem::size_of_val(&size1)); // size1 is 8
    println!("size2 is {}", std::mem::size_of_val(&size2)); // size2 is 8

    let elem = 2u8;
    let mut vec = Vec::new();

    vec.push(elem);

    println!("vec is {:?}", vec); // vec is [2]

    // new type must be CamelCase
    type TestA = u32;
    type TestB = TestA;
    #[allow(non_camel_case_types)]
    type test_c = u32;

    let a: TestA = 8 as test_c;
    let b: TestB = 7 as test_c;

    println!("a + b = {}", a + b); // a + b = 15

}
