type Kilometers = i32;

use std::fmt;
use std::io::Error;

// custom type result T
type Result<T> = std::result::Result<T, std::io::Error>;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: &mut fmt::Arguments) -> Result<()>;
}

// impl<T> Option<T> {
//     pub fn unwrap(self) -> T {
//         match self {
//             Some(val) => val,
//             None => panic!("called `Option::unwrap()` on a `None` value"),
//         }
//     }
// }

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    let guess = "5";
    // let guess: u32 = match guess.trim().parse() {
    //     Ok(num) => num,
    //     Err(_) => continue,
    // };

    // cannot return multiple type
    // let guess = match guess.trim().parse() {
    //     Ok(_) => 5,
    //     Err(_) => "hello",
    // };
}
