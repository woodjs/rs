fn main() {
    //! rust强制RAII(Resource Acquisition Is Initialization)
    //! rust中变量默认将值存放在stack中（包括struct等），可以使用Box<T>将值存放在堆中，无论值存放在哪，都遵守RAII规则
    //! 命令行中，valgrind execName，可查看当前程序内存使用情况
    //! 借用，避免了资源所有权的问题，且当该资源被其他变量引用，该资源就不可被销毁
    //! &，在同一个作用域可多次借用，&mut，在同一个作用域只能借用一次

    {
        let a = Box::new(5i32);

        println!("a is {}", a);
    }

    // println!("a is {}", a); // 报错

    let x = 1;
    let x1 = x; // 值拷贝，不存在所有权问题

    println!("x is {}, x1 is {}", x, x1); // x is 1, x1 is 1

    let ref x2 = x;
    let x3 = &x;

    println!("x2 is equal with x3? {}", *x2 == *x3); // x2 is equal with x3? true

    let mut y = Box::new(2u32);
    let mut y1 = y; // 资源地址拷贝，资源所有权变更，目前，y1拥有所有权
    // let y2 = y; // 报错

    *y1 = 11;

    // println!("y is {}", y); // 报错
    println!("y1 is {}", y1); // y1 is 11

    borrow_val(&mut y1); // afrer this line, the data 12 will not be destroy!
    borrow_val(&mut y1); // afrer this line, the data 13 will not be destroy!

    // 所有权变更
    destroy_val(y1); // afrer this line, the data 13 will be destroy!

    // println!("cur y1 is {}", y1); // 报错
}

// 注意，Box不可省略
fn destroy_val(i: Box<u32>) { // RAII规则，离开作用域时，资源被销毁
    println!("afrer this line, the data {} will be destroy!", i);
}

// 注意，Box可省略
fn borrow_val(i: &mut u32) {
    *i += 1;
    println!("afrer this line, the data {} will not be destroy!", i);
}
