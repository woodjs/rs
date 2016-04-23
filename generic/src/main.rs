fn main() {
    use std::fmt::{Debug, Display};
    // 幽灵类型，编译时存在，运行时不存在
    use std::marker::PhantomData;

    // 新类型A
    struct A;
    // 新类型Test
    struct Test(A);
    // 新泛型类型Gen
    struct Gen<T>(T,);
    // SomeType编译时存在，运行时不存在
    #[derive(Debug, Clone, Copy)]
    struct Phantom<SomeType>(i32, PhantomData<SomeType>);

    let _t = Test(A);
    let _g: Gen<char> = Gen('a');

    fn test<T>(i: Gen<T>) {
        println!("i is a generic type！");
    }

    // 显式定义类型并调用
    test::<u32>(Gen(2)); // i is a generic type！
    // 泛型调用
    test(Gen("test")); // i is a generic type!

    // 显式定义类型
    impl Gen<i32> {
        fn print_value(&self) -> &i32 {

            &self.0
        }
    }

    // 泛型
    impl<T> Gen<T> {
        fn print_gen_value(&self) -> &T {

            &self.0
        }
    }

    let a = Gen(2);

    println!("value is {}, generic value is {}", a.print_value(), a.print_gen_value()); // value is 2, generic value is 2

    // 泛型
    trait DoubleDrop<T> {
        fn double_drop(self, _: T);
    }


    impl<T, U> DoubleDrop<T> for U {
        fn double_drop(self, _: T) {
            println!("curent line is double_drop print!");
        }
    }

    a.double_drop(3); // curent line is double_drop print!

    trait test_bound {
        fn test_bound(&self);
    }

    impl test_bound for A {
        fn test_bound(&self) {
            println!("test trait bound!");
        }
    }

    trait test_multi_bound {
        // 在trait中，type并非定义类型别名的意义
        type A;
        type B;

        fn test_multi_bound(&self);

        // 注意，2，3两个参数，self首字母大写
        fn test_associated_type(&self, &Self::A, &Self::B);
    }

    impl test_multi_bound for A {

        type A = i32;
        type B = i32;

        fn test_multi_bound(&self) {
            println!("test test_multi_bound!");
        }

        fn test_associated_type(&self, num1: &i32, num2: &i32) {
            println!("type A is {}, type B is {}", num1, num2);
        }
    }

    let b = A;
    let c = A;


    // 泛型约束，约束类型必须是trait，i32等类型不是trait
    // +，泛型约束中，所有的trait必须都被实现
    fn show_bound<T: test_bound + test_multi_bound>(i: T) {
        println!("show trait bound!");
    }

    fn where_bound<T>(i: T) where
        T: test_bound + test_multi_bound {
            println!("show where bound!");
        }

    b.test_bound(); // test trait bound!
    show_bound(b); // show trait bound!
    where_bound(c); // show where bound!
    A.test_associated_type(&1, &2); // type A is 1, type B is 2
}
