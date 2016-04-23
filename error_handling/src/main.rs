fn main() {
    //! panic!
    //! 在std标准库中，存在一个枚举Option<T>
    //! Option<T>代表着两种值
    //! Option<T>有一个unwrap方法，从Option中取值
    //! Some(T): An element of type T was found
    //! None: No element was found

    fn test_option(i: Option<&str>) {
        // 对None值unwrap，会报错
        let temp = i.unwrap();

        println!("> current value is {:?}", temp);

        if temp == "a" {
            panic!("error message!");
        }
    }

    let a = Some("a");
    let b = None;

    test_option(a);
    test_option(b);
}
