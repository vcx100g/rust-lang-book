use std::io;

fn main() {
    // guess must have type when parse
    let guess: u32 = "42".parse().expect("Not a number");
    println!("guess: {}", guess);

    // float
    let x: f64 = 2.0;
    let y: f32= 3.0;
    println!("x: {}, y: {}", x, y);

    // boolean
    let t:bool = true;
    let f:bool = false;
    println!("t: {}, f: {}", t, f);

    // char
    let c:char = 'c';
    let z:char = 'z';
    let heart_eyed_cat:char = '😻';
    println!("c: {}, z: {}, heart_eyed_cat: {}", c, z, heart_eyed_cat);

    // array
    let months:[&str; 12] = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    println!("Month: {}", months[10]);
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array item: {}", a[1]);

    // custom array items
    let threes = [3; 5];
    println!("threes: {}", threes[0]);

    // default value
    let a = [123, 245, 3642, 4321, 5789];
    let mut index = String::new();

    println!("Please enter an array index.");

    io::stdin().read_line(&mut index).expect("Failed to read line");

    // auto data type
    let index: usize = index.replace("\n", "").trim().parse().expect("index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );




    // numeric operator
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;
}
/*
Length	Signed	Unsigned
8-bit	i8	    u8
16-bit	i16	    u16
32-bit	i32	    u32
64-bit	i64	    u64
128-bit	i128	u128
arch	isize	usize
 */
/*
Number literals	Example
Decimal	    98_222
Hex	        0xff
Octal	    0o77
Binary	    0b1111_0000
Byte (u8 only)	b'A'
 */