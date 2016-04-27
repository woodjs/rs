fn main() {
    //! thread
    //! channel，线程间异步通信
    //! path，内部表现为Vec<utf8>
    //! file i/o，Result<T,io::Error>别名是io::Result<T>
    //! child process
    //! filesystem operation
    //! program arguments，match可用来解析简单的argument
    //! foreign function interface(FFI)
    //! Foreign functions must be declared inside an extern block annotated with a #[link] attribute containing the name of the foreign library
    //! this extern block links to the libm library
    //! #[link(name = "m")]
    //! extern {
    //!     // this is a foreign function
    //!     fn csqrtf(z: Complex) -> Complex;
    //! }
    //! 必须在unsafe代码块中调用foreign function

    use std::thread;
    use std::sync::mpsc;
    use std::sync::mpsc::{Sender, Receiver};
    use std::path::Path;
    use std::error::Error;
    use std::fs;
    use std::fs::{File, OpenOptions};
    use std::io;
    use std::io::prelude::*;
    use std::process::{Command, Stdio};
    use std::env;

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

    let args: Vec<String> = env::args().collect();

    println!("my path is {}", args[0]); // my path is target\debug\std_misc.exe
    println!("i got {} arguments: {:?}", args.len() - 1, &args[1..]); // i got 0 arguments: []

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

    let output = Command::new("rustc")
                    .arg("--version")
                    .output().unwrap_or_else(|e| {
                        panic!("failed to execute process: {}", e);
                    });

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);

        println!("rustc succeeded and stdout was: {}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);

        println!("rustc failed and stderr was: {}", s);

        // rustc succeeded and stdout was: rustc 1.5.0 (3d7cd77e4 2015-12-04)
    }

    // wait
    let _process = Command::new("sleep").arg("10").spawn();

    println!("reached end of main!");

    // pipe
    match Command::new("wc")
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn() {
                        Err(why) => panic!("couldn't spawn wc: {}", Error::description(&why)),
                        Ok(_) => println!("spawn wc succeeded!")

                        // thread '<main>' panicked at 'couldn't spawn wc: os error', src\main.rs:114
                        // Process didn't exit successfully: `target\debug\std_misc.exe` (exit code: 101)
                    };
}
