fn main() {

    //! variable binding
    //! expression with ';'

    let x = 5u32;

    let y = {

        let a = x;
        let b = 2;

        a + b
    };

    let z = {

        let a = 3 * x;

        a;

    };

    println!("x is {:?}", x); // x is 5
    println!("y is {:?}", y); // y is 7
    println!("z is {:?}", z); // z is ()

}
