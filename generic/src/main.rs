fn main() {

    // 新类型A
    struct A;
    // 新类型Test
    struct Test(A);
    // 新泛型类型Gen
    struct Gen<T>(T,);

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
}
