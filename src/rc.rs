use std::rc::Rc;

#[test]
pub fn test_rc() {

    let val = Rc::new("test rc");

    let val_1 = val.clone();
    let val_2 = val.clone();
    let val_3 = Rc::new("test rc");

    assert_eq!(val_1, val_2);
    assert_eq!(val_1, val_3);
}