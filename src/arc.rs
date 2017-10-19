use std::sync::Arc;
use std::thread;

#[test]
fn test_arc() {

    let val = Arc::new("abc");

    for i in 1..10 {

        let val_clone = val.clone();

        thread::spawn(move || {

            assert_eq!(Arc::try_unwrap(val_clone), "abc");
        });
    }
}