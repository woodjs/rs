use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::thread;

pub fn test_mutex() {

    let val = Arc::new(Mutex::new("mutex"));
    let (sender, receiver) = channel();

    for i in 0..10 {

        let (sender, val) = (sender.clone(), val.clone());

        let thread_result = thread::spawn(move || {
            let data = val.lock().unwrap();

            println!("test mutex thread {}: {:?}", i, *data);
            sender.send(i).unwrap();
        });

        thread_result.join().unwrap();
    }

    receiver.recv().unwrap();
}