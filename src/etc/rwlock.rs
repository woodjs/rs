use std::sync::RwLock;

pub fn test_rwlock() {

    let val = RwLock::new("rwlock");

    {
        let a = val.read().unwrap();
        let b = val.read().unwrap();

        println!("{:?}", *a);
        println!("{:?}", *b);
    }

    {
        let mut val_1 = val.write().unwrap();
        *val_1 = "changed rwlock";

        println!("{:?}", *val_1);
    }
}