use std::cell::Cell;

#[test]
fn test_cell() {

    let val = Cell::new(5);

    let v_1 = val.get();

    assert_eq!(v_1, 5);

    val.set(10);

    let v_2 = val.get();

    assert_eq!(v_2, 10);
}