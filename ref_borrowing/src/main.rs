/*
 &variables
 borrow variables cannot change

 &mut variables
 borrow variables can change

 everytime each borrow happen once
 if borrow to 2 variables, then cannot change
 borrow 1 mutable and 1 non-mutable variable
 wil cause error update
 */
fn main() {
    let s1 = String::from("hello");

    // new method: reference, do not drop after function end
    let len = calculate_length(&s1);

    println!("Length: {}", len);

    let mut t = String::from("horizon");
    // new method: &mut for change variables inside function
    change(&mut t);
    println!("t: {}", t);

    // not mutable cannot be borrow more than once
    // let r = String::from("hello");

    // mutable can borrow more than once
    let mut r = String::from("hello");
    let r1 = &mut r;  // first borrow
    let r2 = &mut r;  // second borrow cause error
    println!("r2: {}", r2); // r1, r2 not longer exists

    // double mutable borrow
    let mut g = String::from("golem");
    {
        let g1 = &mut g;
        println!("g1: {}", g1); // first borrow
    }
    let g2 = &mut g;
    println!("g2: {}", g2); // second borrow, g g1 not longer exists

    // reference borrow, k1 k2 cannot use with k3 together
    let mut k = String::from("hello");
    let k1 = &k;
    let k2 = &k;
    // cause error, k is borrowed and no reference
    // let k3 = &mut k;

    println!("k: {}, k1: {}, k2: {}", k, k1, k2);

    // borrow can work if k1 k2 used then k3 update
    let mut s = String::from("mutable reference 3");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    // reference inside function to outside scope
    let reference_to_nothing = no_dangle();
    println!("no dangle: {}", reference_to_nothing);
}

// use & for reference
fn calculate_length(s: &String) -> usize {
    s.len()
}  // no ownership of s, just reference

fn change(some_string: &String) {
    // not work, because reference cannot modify it
    // some_string.push_str(", world");
}

// this will cause error, because string inside scope is destroyed
// fn dangle() -> &String {
// fn dangle() -> &String {
//     let s = String::from("world");
//     &s
// }

// this will work for s is full memory object to outside scope
fn no_dangle() -> String {
    let s = String::from("world");
    s
}