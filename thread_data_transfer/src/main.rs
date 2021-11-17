use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    // channel tx sender, rx receiver
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("first");
        // send value to channel
        tx.send(val).unwrap();

        // cause error because val is already borrow to channel
        // println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // sends and received multiple variables
    let (tx1, rx1) = mpsc::channel();

    // for 2 thread, you need to use multiple sender
    let tx2 = tx1.clone();

    thread::spawn(move || {
        let values = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in values {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // move tx2 in scope to send
    thread::spawn(move || {
        let values = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in values {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx1 {
        println!("Got: {}", received);
    }
}
