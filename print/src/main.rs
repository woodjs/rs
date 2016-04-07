use std::fmt::{self, Display, Formatter};

struct Structure(i32, i32);
struct List(Vec<i32>);

/// 扩展Structure结构体，如果想要改变":b"默认定义，需要impl fmt::Binary
impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1) // write!，写入流
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let List(ref vec) = *self;

        try!(write!(f, "["));

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                try!(write!(f, ", "));
            }

            try!(write!(f, "{}", v));
        }

        // 最后一步，不可使用try!，因为返回值要匹配fmt::Result类型
        write!(f, "]")
    }
}

fn main() {
    //! std::fmt
    //! std::fmt::Debug => {:?}
    //! std::fmt::Display => {}

    // {:?} => "bugong"，{} => bugong
    println!("hello rust, my name is {:?}", "bugong"); // hello rust, my name is "bugong"
    println!("age: {0}", 26); // age: 26
    // 定义特殊的格式化，":b, :x, :o, :.3(保留3位小数)"
    println!("age to binary: origin -> {}, parsed -> {:b}", 26, 26); // age to binary: origin -> 26, parsed -> 11010
    println!("language: {lang1}, {lang2}", lang1 = "chinese", lang2 = "english"); // language: chinese, english
    println!("work: format1 -> {number:>width$} years", number = 3, width = 8); // work: format1 ->        3 years
    // 用不是0的数字填充报错
    println!("work: format2 -> {number:>0width$} years", number = 3, width = 8); // work: format2 -> 00000003 years

    #[derive(Debug)]
    struct Test(i32);
    // 未扩展时，必须使用{:?}，使用{}报错
    println!("this will print {:?} without error!", Test(26)); // this will print Test(26) without error!
    //扩展后
    println!("this will print {} wihout error!", Structure(32, 64)); // this will print (32, 64) wihout error!

    let v = List(vec![1, 2, 3]);

    println!("this will print {} wihtout error!", v); // this will print [1, 2, 3] wihtout error!
}
