/*
box use for content another variables without knowing what size or how long
usually use for recursion function type
 - type whose size can’t be known and use a value type context exact size
 - large amount of data and transfer ownership but ensure won’t be copied
 - own a value and it’s a type that implements trait rather than specific type
 */
// new method: deref using *

use crate::List::{Cons, Nil};

// recursion wont work for list
// enum List {
//     Cons(i32, List),
//     Nil
// }

// if content in box will work
// box is just a pointer to another memory location
enum List {
    Cons(i32, Box<List>),
    Nil
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new (x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl <T> Deref for MyBox<T> {
    type Target = T;

    // if got remove reference then return as self
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello: {}!", name);
}

fn main() {
    let b = Box::new(5);
    println!("b: {}", b);

    // this cause error because recursion type have unknown size
    // let list = Cons(1, Cons(2, Cons(3, Nil)));

    // this will work because box content recursion list
    let list = Cons(
        1,
        Box::new(Cons(
            2,
            Box::new(Cons(
                3,
                Box::new(Nil))
            ))
        ));

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    // this cause error because already x not borrow to y
    // assert_eq!(5, y);

    // using * deref: remove reference to allow y break connecting with x
    assert_eq!(5, *y);

    let x1 = 5;
    let y1 = MyBox::new(x);

    assert_eq!(5, x1);
    assert_eq!(5, *y1);

    // use string inside box pointer
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // remove reference for m box; then put m string reference
    hello(&(*m)[..]);
}

/*
deref when using dynamic type
    From &T to &U when T: Deref<Target=U>
    From &mut T to &mut U when T: DerefMut<Target=U>
    From &mut T to &U when T: Deref<Target=U>
 */