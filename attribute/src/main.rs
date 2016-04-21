extern crate attribute;

fn main() {
    //! #![crate_attribute]，应用到整个crate
    //! #[item_attribute]，应用到一个item

    //! #[attribute = "value"]
    //! #[attribute(key = "value")]
    //! #[attribute(key)]

    //! cfg(conditional compilation)
    //! attribute: #[cfg(key)]
    //! macro_rules: cfg!(boolean expression)

    //! 自定义conditional，必须使用--cfg标签，即rustc --cfg custom_conditional_name

    #[allow(dead_code)]
    fn unused() {}

    #[cfg(target_os = "linux")]
    fn print_os() {
        println!("you are running on linux!");
    }

    #[cfg(not(target_os = "linux"))]
    fn print_os() {
        println!("you are running on windows!");
    }

    print_os(); // you are running on windows!

    if cfg!(target_os = "linux") {
        println!("linux print!");
    } else {
        println!("windows print!"); // windows print!
    }

    attribute::print_test(); // this a attribute test!
}
