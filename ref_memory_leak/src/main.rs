/*
 Weak reference
 */

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum List {
    // cannot refCell outside of reference counter
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

#[derive(Debug)]
struct Node {
    value: i32,
    // use weak to reference to parent
    parent: RefCell<Weak<Node>>,
    // ref cell outside of vector then ref counter work
    children: RefCell<Vec<Rc<Node>>>,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack, loop forever
    // println!("a next item = {:?}", a.tail());

    // using weak ref
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()), // use weak here
        children: RefCell::new(vec![]),
    });

    // borrow parent and its none
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // mutable leaf parent variables set to branch weak reference
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    // it work and no stack overflow loop forever
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
