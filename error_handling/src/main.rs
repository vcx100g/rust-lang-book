/*
recoverable and unrecoverable errors
add RUST_BACKTRACE=full to environment
 */

use std::fs::File;
use std::io::ErrorKind;
use error_handling::*;

// enum error handling
enum Result<T, E> {
    Ok(T),
    Err(E)
}

fn main() {
    // testing system panic then abort
    // panic!("crash and burn");

    // let v = vec![1, 2, 3, 4, 5];
    // v[99];

    // let f:u32 = File::open("hello.txt");

    // basic error handling when opening file
    // let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Error opening file: {:?}", error)
    // };

    // more error details here, error kind, create new file
    let f2 = File::open("hello2.txt");
    let f2 = match f2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello2.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error creating file: {:?}", e)
            },
            other_error => {
                panic!("Error opening file: {:?}", other_error)
            }
        }
    };

    // without using match, use if else
    let f3 = File::open("hello3.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello3.txt").unwrap_or_else(|error| {
                panic!("Error creating file: {:?}", error);
            })
        } else {
            panic!("Error opening file: {:?}", error);
        }
    });

    // unwarp: if error call panic
    // let f4 = File::open("hello4.txt").unwrap();

    // expect: throw if error found
    // let f5 = File::open("hello5.txt").expect("Error opening file");

    let result = read_username_from_file();
    dbg!(result);

    let result2 = read_username_from_file2();
    dbg!(result2);

    let result3 = read_username_from_file3();
    dbg!(result3);

    let result4 = read_username_from_file4();
    dbg!(result4);
}

// dyn error? chapter 17
// use std::error::Error;
//
// fn main2() -> Result<(), Box<dyn Error>> {
//     let f = File::open("hello.txt")?;
//     Ok(())
// }