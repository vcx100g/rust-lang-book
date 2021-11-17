// comment
/*
better comment
 */

fn main() {
    println!("Hello, world!");

    another_function(5);
    print_label_measurement(8, 'h');

    // new code: bracket group value
    let y = {
        let y = 6;
        y + 1 // no close ;
    };
    println!("y: {}", y);

    let x = five();
    println!("Value of five: {}", x);

    let two = plus_one(1);
    println!("One plus one: {}", two);
}

fn another_function(x: i32) {
    println!("Another function");
    println!("x: {}", x);
}

fn print_label_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

// declare return value type function
fn five() -> i32 {
    5  // no close
}

// parameters and return type
fn plus_one(x: i32) -> i32 {
    x + 1 // no close ; if you put will error
}

// plus two function > copilot good
fn plus_two(x: i32) -> i32 {
    x + 2
}

