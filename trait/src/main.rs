fn main() {
    //! trait，定义在未知类型Self上的方法集合
    //! 通过使用#[derive]，可以自动为某些trait添加基础的实现

    //! 可derive的trait如下：
    //! Comparison traits: Eq, PartialEq, Ord, PartialOrd
    //! Clone, to create T from &T via a copy
    //! Hash, to compute a hash from &T
    //! Default, to create an empty instance of a data type
    //! Zero, to create a zero instance of a numeric data type
    //! Debug, to format a value using the {:?} formatter

    //! 重载

    //! trait Drop只定义了一个方法drop，该方法在对象离开作用域时自动调用，已为Box, Vec, String, File, Process等类型默认实现
    //! trait Iterator，需手动实现next方法（array和range中已自动实现）
    //! trait Clone，有一个clone方法

    use std::ops;

    #[derive(PartialEq, Clone, Debug)]
    struct Test(i32);
    struct Element;
    #[derive(Debug)]
    struct Value;

    let a = Test(1);
    let b = Test(1);

    println!("a is equal with b? {}", a == b); // a is equal with b? true

    impl ops::Add<Element> for Test {
        type Output = Value;

        fn add(self, _: Element) -> Value {
            Value
        }
    }

    let c = Element;

    let d = a.clone();


    println!("a + b = {:?}", a + c); // a + b = Value

    // drop，默认已实现
    drop(b);

    // println!("current b is {:?}", b); // 报错
    println!("current d is {:?}", d); // current d is Test(1)
}
