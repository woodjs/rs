use std::sync::Arc;
use std::thread;

#[test]
fn test_arc() {

    let val = Arc::new("arc");

    for i in 0..10 {

        let val_clone = val.clone();

        thread::spawn(move || {

            println!("test arc thread {:?}:", i);
        });
    }
}