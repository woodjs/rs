macro_rules! say_hello {
    () => (
        println!("hello, rust!");
    )
}

fn main() {
    say_hello!(); // hello, rust!
}
