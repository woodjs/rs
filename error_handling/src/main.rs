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

    enum A {
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

        opt.and_then(and_then_print);
    }

    fn and_then_print() -> Option<A> {
        println!("hello, {:?} is printed by and_then!", 123);
        Some(A::A);
    }

    fn create_option() -> Option<A> {
        let a = Some(A::A);
        a
    }

    let a = Some("b");
    // let b = None; // None值必须被使用，否则报错

    test_option_unwrap(a);
    //test_option_unwrap(b); // 对None值unwrap，会报错

    test_option_map(a); // the value is b!

    test_option_and_then();

}
