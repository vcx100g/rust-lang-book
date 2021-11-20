/*
    why use unsafe
    - Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
    - Aren’t guaranteed to point to valid memory
    - Are allowed to be null
    - Don’t implement any automatic cleanup

    mostly used for
    - interact with code written in another language
    - custom memory allocation
 */

unsafe fn dangerous() {
    println!("Unsafe function");
}

use std::ptr::slice_from_raw_parts_mut;
use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid)
        )
    }
}

// use C variables
extern "C" {
    fn abs(input: i32) -> i32;
}

// create C connector
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// static constants can modify using unsafe
static HELLO_WORLD: &str = "Hello, world!";

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// unsafe trait and implementation
unsafe trait Foo {}

unsafe impl Foo for i32 {}

fn main() {
    let mut num = 3;

    // unsafe constants
    let r1 = &num as *const i32;

    // unsafe mutable variables
    let r2 = &mut num as *mut i32;

    let address = 0x012345usize;
    let r = address as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);

        // cannot print memory address
        // println!("r is: {}", *r);
    }

    // run unsafe function
    unsafe {
        dangerous()
    }

    // create and reference for unsafe vector
    let mut v = vec![1, 2, 3, 4, 5];
    let r = &mut v[..];

    // split unsafe vector into tuple
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5]);

    // create and reference for unsafe blank memory
    let address2 = 0x012345usize;
    let r = address as *mut i32;

    // cut memory put into slice
    let slice: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };

    // connect to C application binary interface (ABI)
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    call_from_c();

    // static constants can modify using unsafe
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
