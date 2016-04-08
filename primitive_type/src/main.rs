use std::mem;

fn main() {
    // primitve 原始类型
    // i8, i16, i32, i64, isize
    // u8, u16, i32, i64, usize
    // f32, f64
    // char
    // bool
    // ()
    // [1, 2, ..]
    // (1, true)

    // literal 字面量
    // 0x, 0o, 0b
    // _，对于number字面量，1_000 == 1000
    // suffix（后缀），1u32

    // operator 运算符
    // &&, ||, !
    // &, |, ^
    // <<, >>

    println!("1 + 2 = {}", 1u32 + 2); // 1 + 2 = 3
    // 1u32 - 2，会报溢出错误
    println!("1 - 2 = {}", 1i32 - 2); // 1 - 2 = -1

    println!("true AND false is {}", true && false); // true AND false is false
    println!("true OR false is {}", true || false); // true OR false is true
    println!("NOT true is {}", !true); // NOT true is false

    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101); // 0011 AND 0101 is 0001
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101); // 0011 OR 0101 is 0111
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101); // 0011 XOR 0101 is 0110
    println!("1 << 5 is {}", 1u32 << 5); // 1 << 5 is 32
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2); // 0x80 >> 2 is 0x20

    println!("one million can written as {}", 1_000_000u32); // one million can written as 1000000

    // 类型注解，: [i32; 5]，可不加
    let arr: [i32; 5] = [1; 5];

    // 原始数组的size，编译时已经知道，切片的size，编译时并不知道
    println!("second element of arr is : {}", arr[1]); // second element of arr is : 1
    println!("arr length is {}", arr.len()); // arr length is 5
    println!("arr occupies {} bytes", mem::size_of_val(&arr)); // arr occupies 20 bytes

}
