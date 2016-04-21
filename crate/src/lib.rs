/// crate，编译单元
/// binary/library，默认binary，通过--crate-type参数指定编译目标格式
/// rustc --crate-type=lib test.rs => libtest.rlib

mod test {
    pub fn test() {
        private_test();
    }

    fn private_test() {
        println!("this is a line, print by private_test!");
    }
}
