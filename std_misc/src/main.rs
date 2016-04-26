fn main() {
    //! thread
    //! channel，线程间异步通信
    //! path，内部表现为Vec<utf8>
    //! file i/o，Result<T,io::Error>别名是io::Result<T>
    //! child process
    //! filesystem operation
    //! program arguments
    //! foreign function interface(FFI)

    use std::thread;
    use std::sync::mpsc;
    use std::sync::mpsc::{Sender, Receiver};
    use std::path::Path;
    use std::error::Error;
    use std::fs::File;
    use std::io::prelude::*;

    // 静态类型变量，必须全部大写
    static NUM: i32 = 3;

    let (sender, receiver): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    for id in 0..NUM {
        let tread_sender = sender.clone();

        thread::spawn(move || {
            tread_sender.send(id).unwrap();

            println!("thread {} finished!", id);
        });

        // thread 0 finished!
        // thread 2 finished!
        // thread 1 finished!
    }

    let mut ids = Vec::with_capacity(NUM as usize);

    for _ in 0..NUM {
        ids.push(receiver.recv());
    }

    println!("{:?}", ids); //[Ok(0), Ok(1), Ok(2)]

    let path = Path::new("out/hello_rust.txt");

    println!("path is {:?}", path.display()); // path is "hello_rust.txt"

    static TXT: &'static str = "hello rust!!!!!!!!!";

    let out_path = Path::new("out/hello_rust.txt");
    let display = out_path.display();

    let mut file = match File::create(&out_path) {
        Err(why) => panic!("couldn't create {}: {}", display, Error::description(&why)),
        Ok(file) => file
    };

    match file.write_all(TXT.as_bytes()) {
        Err(why) => panic!("couldn'd write to {}: {}", display, Error::description(&why)),
        Ok(_) => println!("successfully wrote to {}", display)

        // successfully wrote to out/hello_rust.txt
    }

    let new_path = path.join("a").join("b").join("c");

    match new_path.to_str() {
        None => panic!("new_path is not a valid UTF-8 sequence"),
        Some(s) => println!("new_path is {}", s)

        // new_path is hello_rust.txt\a\b\c
    }

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), Error::description(&why)),
        Ok(file) => file
    };

    let mut s = String::new();

    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn'd read {}: {}", path.display(), Error::description(&why)),
        Ok(_) => println!("{} contains:\n{}", path.display(), s)

        // out/hello_rust.txt contains:
        // hello rust!!!!!!!!!
    }
}
