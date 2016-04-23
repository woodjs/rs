fn main() {
    //! rust强制RAII(Resource Acquisition Is Initialization)
    //! rust中变量默认将值存放在stack中（包括struct等），可以使用Box<T>将值存放在堆中，无论值存放在哪，都遵守RAII规则
    //! 命令行中，valgrind execName，可查看当前程序内存使用情况
    //! 借用，避免了资源所有权的问题，且当该资源被其他变量引用，该资源就不可被销毁
    //! &，在同一个作用域可多次借用，&mut，在同一个作用域只能借用一次
    //! 生命周期，为引用服务，长生命周期可以注入到短生命期，但是短生命周期不可以注入到长生命周期
    //! 未自定义添加的生命周期标志，rust在编译时会自动添加，以增加程序代码可读性
    //! 'static生命周期，1：static关键字，2：对于stirng，定义类型为&‘static str

    {
        let a = Box::new(5i32);

        println!("a is {}", a);
    }

    // println!("a is {}", a); // 报错

    let x = 1;
    let x1 = x; // 值拷贝，不存在所有权问题

    destroy_stack_val(x1); // afrer this line, the stack data 1 will not be destroy!

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

    #[derive(Debug)]
    struct LifeTimeStruct<'a>(&'a i32);

    #[derive(Debug)]
    enum LifeTimeEnum<'a> {
        Test(i32),
        Ref(&'a i32)
    }

    let a = 1;
    let b = 2;

    let s = LifeTimeStruct(&a);
    let t = LifeTimeEnum::Test(a);
    let r = LifeTimeEnum::Ref(&b);

    println!("s is {:?}, t is {:?}, r is {:?}", s, t, r); // s is LifeTimeStruct(1), t is Test(1), r is Ref(2)
}

// 注意，Box不可省略
fn destroy_val(i: Box<u32>) { // RAII规则，离开作用域时，资源被销毁
    println!("afrer this line, the data {} will be destroy!", i);
}

fn destroy_stack_val(i: i32) { // RAII规则，离开作用域时，资源被销毁
    println!("afrer this line, the stack data {} will not be destroy!", i);
}

// 注意，Box可省略
fn borrow_val<'a>(i: &'a mut u32) {
    *i += 1;
    println!("afrer this line, the data {} will not be destroy!", i);
}
