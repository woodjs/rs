use std::rc::Rc;
use std::rc::Weak;

#[test]
fn test_rc() {

    let val = Rc::new("test rc");

    let val_1 = val.clone();
    let val_2 = val.clone();
    let val_3 = Rc::new("test rc");

    assert_eq!(val_1, val_2);
    assert_eq!(val_1, val_3);
}

#[test]
fn test_weak_rc() {

    let val = Rc::new("abc");
    let val_1: Weak<&str> = Rc::downgrade(&val);
    let val_2 = val.clone();
    let val_3 = val_1.upgrade();

    assert_eq!(val, val_2);
    assert_eq!(val, val_3.unwrap());
}