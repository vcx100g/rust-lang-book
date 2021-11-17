/*
 thread::spawn create new thread in os; do not work in wasm
 move: use out of scope parent variable in thread spawn
 */

use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // if put here will finish then run next line
    // handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    };

    // make sure thread finish
    handle.join().unwrap();

    let v = vec![1, 2, 3, 4, 5];

    let handle2 = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // drop(v);

    handle2.join().unwrap();
}
