fn main() {
    //! panic!

    //! 在std标准库中，存在一个枚举Option<T>
    //! Option<T>代表着两种值：
    //! Some(T): An element of type T was found
    //! None: No element was found
    //! Option<T>内置的方法：
    //! unwrap，从Option中取值
    //! map，可链式调用，代替嵌套match
    //! and_then，处理嵌套Option，如Option<Option<T>>，代替复杂的match

    //! Result<T, E>：
    //! Ok<T>: An element T was found
    //! Err<E>: An error was found with element E
    //! Result<T, E>内置的方法：
    //! unwrap
    //! map
    //! map_err
    //! and_then

    //! try!，与unwrap类似，不同的是，用error代替panic

    //! Box<Error>

    use std::num::ParseIntError;
    use std::result;

    // 定义别名
    type Result<T> = result::Result<T, ParseIntError>;

    struct A(i32);

    enum B {
        A
    };

    // Option中参数的类型必须明确
    fn test_option_unwrap(i: Option<&str>) {
        let temp = i.unwrap();

        println!("> current value is {:?}", temp);

        if temp == "a" {
            panic!("error message!");
        }
    }

    fn test_option_map(i: Option<&str>) {
        i.map(|i| { i })
            .map(|info| {
                println!("the value is {}!", info);
            });
    }

    fn test_option_and_then() {
        let opt = create_option();

        // opt.and_then(and_then_print);
    }

    fn and_then_print() -> Option<B> {
        println!("hello, {:?} is printed by and_then!", 123);
        Some(B::A)
    }

    fn create_option() -> Option<B> {
        Some(B::A)
    }

    fn double_number(i: &str) -> Result<i32> {
        match i.parse::<i32>() {
            Ok(n) => Ok(2 * n),
            Err(e) => Err(e)
        }
    }

    fn double_number_map(i: &str) -> Result<i32> {
        i.parse::<i32>().map(|n| { 2 * n })
    }

    fn print_result(r: Result<i32>) {
        match r {
            Ok(n) => println!("current number is {}!", n),
            Err(e) => println!("error info is : {}!", e)
        }
    }

    let a = Some("b");
    // let b = None; // None值必须被使用，否则报错

    test_option_unwrap(a); // > current value is "b"
    //test_option_unwrap(b); // 对None值unwrap，会报错

    test_option_map(a); // the value is b!

    //test_option_and_then

    let a = double_number("10");
    let b = double_number_map("20");
    let c = double_number_map("a");

    print_result(a); // current number is 20!
    print_result(b); // current number is 40!
    print_result(c); // error info is : invalid digit found in string!

    // let d = create_some().unwrap();
    let d = try!(create_some());

    fn create_some() -> Result<i32> {
        let i = "123";

        i.parse::<i32>()
    }

    println!("d is {}", d);
}
