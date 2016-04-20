mod my {
    fn private_test() {
        println!("this default private function test!");
    }

    pub fn test() {
        println!("this a function test defined by pub!");
    }

    mod private_sub_mod {
        fn test() {
            println!("this a private_sub_mod test!");
        }
    }

    pub mod sub_mod {
        fn private_test() {
            println!("this a sub_mode private_test!");
        }

        pub fn test() {
            println!("this a sub_mode test!");
        }

        pub fn key_word() {
            self::private_test();
            super::private_test();
        }
    }

    pub struct Test<T> {
        pub content: T
    }

    // impl后面的<T>，不可省略
    impl<T> Test<T> {
        pub fn new(content: T) -> Test<T> {
            Test {
                content: content
            }
        }
    }
}

fn main() {

    //! 模块，最小逻辑单元
    //! private/public
    //! 模块可嵌套，模块内部元素，默认private
    //! 内部模块，可访问外部模块的private元素
    //! pub关键字
    //! mod内，struct字段默认private
    //! use关键字
    //! self关键字，当前模块
    //! super关键字，父模块
    //! 模块文件系统
    //! 一个模块可以分割为多个文件，文件夹名即为模块名，每个子文件名对应一个子空间名

    // use必须写在代码块第一句
    use my::test as say;

    say(); // this a function test defined by pub!
    my::test(); // this a function test defined by pub!
    my::sub_mod::test(); // this a sub_mode test

    let s = my::Test::new("rust!");

    println!("struct content is {}", s.content); // struct content is rust!

    my::sub_mod::key_word();
    // this a sub_mode private_test!
    // this default private function test!

}
