/*
 memory safety guarantees
 without garbage collection
 allocating on the heap
 */
/*
 - Each value in Rust has a variable that’s called its owner.
 - There can only be one owner at a time.
 - When the owner goes out of scope, the value will be dropped.
 */
fn main() {
    // owner is main fn
    let mut s = String::from("hello");
    s.push_str(", owner!");
    println!("{}", s);

    // same memory heap allocating
    let s1 = String::from("hello");
    let s2 = s1;
    // not work, s1 already point to s2
    // println!("{}, world", s1);

    // new method: need to clone variable
    let s1 = String::from("hello");
    let s3 = s1.clone();
    println!("s1: {}, s3: {}", s1, s3);

    // take ownership
    let s = String::from("new world");
    // s have inside function, s not valid outside
    takes_ownership(s);

    let x = 5;
    // i32 is a copy, valid outside too
    makes_copy(x);

    // note: string and integer are difference

    let s1 = gives_ownership();

    let s2 = String::from("old world");

    // after s2 give to function, function return to s3
    // s2 not longer exists
    let s3 = takes_and_gives_back(s2);

    println!("s1: {}, s3: {}", s1, s3);


    let s1 = String::from("god fall");
    // same as above, variables inside function not longer exists
    // s1 is dropped after function close
    let (s2, len) = calculate_length(s1);

    println!("Length of '{}' is {}.", s2, len);

} // this scope is now over, and s is no longer valid

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
// some_string is out of scope and drop is called
// backing memory is freed

fn makes_copy(some_interface: i32) {
    println!("{}", some_interface);
}
// integer goes out of scope and still valid outside

// return a string, variable move out function
fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

// take a string parameters then return it ownership
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

// return tuple
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}