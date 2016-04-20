/// extern crate关键字，导入外部的library

extern crate libtest as abc;

fn main() {
    abc::test();
}
