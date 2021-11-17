use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // share variables across threads
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // spawn 10 threads
    for _ in 0..10 {
        // clone variables to thread
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            // remove reference then calculate
            *num += 1
        });

        // push back to vector
        handles.push(handle);
    }

    // wait all thread finish
    for handle in handles {
        handle.join().unwrap();
    }

    // remove reference lock then get variables
    println!("Result: {}", *counter.lock().unwrap());
}
