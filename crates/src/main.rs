/// extern crate关键字，导入外部的library

extern crate crates as abc;

fn main() {
    abc::test::test(); // this is a line, print by private_test!
}
