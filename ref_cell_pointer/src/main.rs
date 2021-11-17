/*
 refCell interior mutable to data
 an array item cannot borrow outside and change
 it need to use refCell to update the borrowed array item

 RefCell<>
 borrow_mut()
 borrow()
 */

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    // use refCell and reference counter together
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil
}

fn main() {
    // create a reference counter for reference cell value
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    // reference counter borrow twice
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // remove reference for value and borrow mutable
    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
