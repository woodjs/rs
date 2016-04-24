//! 当执行Rustdoc时，用///注释的部分将被编译生成为文档，支持markdown语法
//! 函数可通过下面几个属性进行单元测试：
//! #[test] marks a function as a unit test. The function must take zero parameters and return nothing.
//! #[should_panic] marks a function as a panicking test
//! 通过执行，cargo test or rustc --test进行测试

#[cfg(not(test))]
fn main() {

    println!("now, is no testing!");
}

#[cfg(test)]
fn main() {

    println!("now, is testing!");
}

#[cfg(test)]
mod test {
    fn num() -> i32 {
        1
    }

    #[test]
    fn test_num_ok() {
        println!("> {:?}", "test_num_ok");
        assert!(num() == 1);
    }

    #[test]
    #[should_panic]
    fn test_num_not_ok() {
        println!("> {:?}", "test_num_not_ok");
        assert!(num() != 1);
    }
}
