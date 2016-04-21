/// extern crate关键字，导入外部的library

extern crate crate1 as abc;

fn main() {
    abc::test::test();
}
