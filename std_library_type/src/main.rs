fn main() {
    //! rust中，所有的值，默认都分配在栈中，可使用Box<T>分配在堆中，返回一个指针，使用*解构Box取值
    //! vector，可增长或收缩的数组，堆上存储，编译时不知道大小
    //! string，实质上就是Vec<utf8>，堆上存储，大小可伸缩
    //! &str，实质上就是&[utf8]，大小固定
    //! T => &T == Vec<T>
    //! Option<T>
    //! None, to indicate failure or lack of value
    //! Some(value), a tuple struct that wraps a value with type T
    //! Result<T, E>，相比于Option，Result可表达为何失败
    //! Ok(value)
    //! Err(why)
    //! try!，解决match嵌套的问题，会展开为对应的Ok或Err分支，如果发生错误会提前返回Result
    //! panic!，会释放所有资源
    //! HashMap，key值可以为实现Eq特性和Hash特性的任意类型，如boolean, integer, string等，大小可伸缩
    //! f32，f64未实现HashMap
    //! 如果容器内的元素类型实现了Eq和Hash，那么这个容器元素也继承实现了Eq和Hash
    //! 可以通过#[derive(PartialEq, Eq, Hash)]，为自定义类型如struct自动实现Eq和Hash
    //! HashSet，a wrapper around HashMap<T, ()>，其关注点为key，key不可重复，4个主要的方法如下：
    //! union: get all the unique elements in both sets
    //! difference: get all the elements that are in the first set but not the second
    //! intersection: get all the elements that are only in both sets
    //! symmetric_difference: get all the elements that are in one set or the other, but not both

    use std::collections::HashMap;

    #[derive(Debug)]
    struct A {
        x: i32
    }

    let a: Box<A> = Box::new(A {x: 1});
    let b = Box::new(&a);

    println!("a is {:?}", a); // a is A { x: 1 }
    println!("b is {:?}", b); // b is A { x: 1 }

    let mut a: Vec<i32> = (1..10).collect();

    println!("a is {:?}", a); // a is [1, 2, 3, 4, 5, 6, 7, 8, 9]
    println!("length of a is {}", a.len()); // length of a is 9
    println!("pop {:?}", a.pop()); // pop Some(9)
    println!("push {:?}", a.push(22)); // push ()
    println!("current a is {:?}", a); // current a is [1, 2, 3, 4, 5, 6, 7, 8, 22]

    let a = Some(1i32);

    println!("a is {}", a.unwrap()); // a is 1

    #[derive(Debug)]
    enum CustomError {
        TestError
    }

    type CustomResult = Result<i32, CustomError>;

    fn test_try(i: i32) -> CustomResult {
        if i == 0 {
            Err(CustomError::TestError)
        } else {
            Ok(i)
        }
    }

    fn print_result(i: i32) -> CustomResult {
        let a = try!(test_try(i));
        Ok(a)
    }

    println!("print_result is {:?}", print_result(0)); // print_result is Err(TestError)
    println!("print_result is {:?}", print_result(11)); // print_result is Ok(11)

    let mut a = HashMap::new();

    a.insert("a", "1");
    a.insert("b", "2");
    a.remove("b");
    a.insert("c", "test");

    println!("a is {:?}", a); // a is {"a": "1", "c": "test"}z
    println!("a.c is {:?}", a.get(&"c")); // a.c is Some("test")
}
