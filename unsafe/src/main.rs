fn main() {
    //! unsafe代码块的四大用处如下：
    //! dereferencing raw pointers
    //! calling a function over FFI
    //! changing types through std::cast::transmute
    //! inline assembly（内联汇编）

    let a: *const u32 = &10;
    let b = 11i32;

    unsafe {
        assert!(*a == 10);
        assert!(std::mem::transmute::<i32, u32>(b) == 11);
    }
}
