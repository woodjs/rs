macro_rules! say_hello {
    () => (
        println!("hello, rust!");
    )
}

macro_rules! print_result {
    // *，该argument重复0次或多次， +，该argument至少重复一次
    ($expression: expr; and $($test: expr) *) => (
        println!("{:?} = {:?}, another argument is {:?}", stringify!($expression), $expression, $($test) *);
    )
}

macro_rules! create_func {
    ($func_name: ident) => (
        fn $func_name() {
            println!("you called function {:?}", stringify!($func_name));
        }
    )
}


fn main() {

    //! 自定义宏，参数名以'$'为前缀，其类型指定为rust定义的一系列designator(标志符)

    //! 所有的designator如下：
    //! block
    //! expr (expression)
    //! ident (variable/function name)
    //! item
    //! pat (pattern)
    //! path
    //! stmt (statement)
    //! tt (token tree)
    //! ty (type)

    say_hello!(); // hello, rust!

    print_result!({
        let a = 2;

        a * a
    }; and {
        let a = 8;
        a
    }); // "{ let a = 2; a * a }" = 4, another argument is 8

    create_func!(foo);

    foo(); // you called function "foo"
}
