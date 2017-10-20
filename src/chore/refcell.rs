use std::cell::RefCell;
use std::cell::Ref;

pub fn test_refcell() {

    let val = RefCell::new(5);

    println!("test refcell: {:?}", val.clone().into_inner());

    {
        let v_1 = val.borrow();
        let v_2 = val.borrow();

        println!("borrow: {:?}", v_1);
        println!("borrow: {:?}", v_2);
    }

    {
        let mut v_1 = val.borrow_mut();

        *v_1 = 10;
        println!("borrow mut: {:?}", v_1);
    }
}