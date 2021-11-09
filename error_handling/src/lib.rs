use std::error::Error;
use std::fs::{self, File};
use std::io::{self, Read};

enum Result<T, E> {
    Ok(T),
    Err(E)
}

pub fn read_username_from_file() -> std::result::Result<String, std::io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// new method: using ? mean throw error if found
// like async await in js but for error
pub fn read_username_from_file2() -> std::result::Result<String, std::io::Error> {
    let mut f = File::open("hello.txt")?; // ? throw if found error
    let mut s = String::new();
    f.read_to_string(&mut s)?; // ? throw if found error
    Ok(s)
}

pub fn read_username_from_file3() -> std::result::Result<String, std::io::Error> {
    let mut s = String::new();

    // best: one line multi throw error output
    File::open("hello.txt")?.read_to_string(&mut s);

    Ok(s)
}

// using builtin function to throw
pub fn read_username_from_file4() -> std::result::Result<String, std::io::Error> {
    fs::read_to_string("hello.txt")
}
