// create system meta programming keyword
macro_rules! say_hello {
    () => {
        println!("hello");
    }
}

fn main() {
    say_hello!();
}
