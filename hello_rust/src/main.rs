fn main() {
    //! std::fmt
    //! std::fmt::Debug => {:?}
    //! std::fmt::Display => {}

    println!("hello rust, my name is {}", "bugong"); //hello rust, my name is bugong
    println!("age: {0}", 26); //age: 26
    //定义特殊的格式化，":b"
    println!("age to binary: origin -> {}, parsed -> {:b}", 26, 26); //age to binary: origin -> 26, parsed -> 11010
    println!("language: {lang1}, {lang2}", lang1 = "chinese", lang2 = "english"); //language: chinese, english
    println!("work: format1 -> {number:>width$} years", number = 3, width = 8); //work: format1 ->        3 years
    //用不是0的数字填充报错
    println!("work: format2 -> {number:>0width$} years", number = 3, width = 8); //work: format2 -> 00000003 years
}
