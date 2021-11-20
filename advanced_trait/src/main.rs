pub trait Iterator {
    type Item;

    // Option for Some or None
    fn next(&mut self) -> Option<Self::Item>;
}

use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    // custom + add operator for struct
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// trait Add<Rhs=Self> {
//     type Output;
//
//     fn add(self, rhs: Rhs) -> Self::Output;
// }

#[derive(Debug, Copy, Clone, PartialEq)]
struct Millimeters(u32);

#[derive(Debug, Copy, Clone, PartialEq)]
struct Meters(u32);

// custom + add operator for struct Millimeters
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + other.0 * 1000)
    }
}

// oop
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

// overlap method in trait
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

use std::fmt;

// custom outline print
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// fmt must be implemented to use OutlinePrint
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

// wrapper for fmt::Display custom print
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    assert_eq!(
        Millimeters(1500) + Meters(2),
        Millimeters(3500)
    );

    // which fly will run?
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    // which baby_name will run? animal or dog
    // cause error because animal dont have baby name
    // println!("A baby {} is called a {}!", Animal::baby_name(), Dog::baby_name());

    // use <Dog as Animal> to call animal
    println!("A baby {} is called a {}!", <Dog as Animal>::baby_name(), Dog::baby_name());

    // outline_print
    let point = Point { x: 3, y: 4 };
    point.outline_print();

    // wrapper
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w); // [hello, world] add to _string()
}
