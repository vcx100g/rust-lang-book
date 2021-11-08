#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

// add method into struct object
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width_exists(&self) -> bool {
        self.width > 0
    }

    // reference another rectangle outside
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// multi block also work
impl Rectangle {
    // create a square
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45
    };

    if rect1.width_exists() {
        println!(
            "The area of the rectangle is {} square pixels.",
            rect1.area()
        );
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // call struct method without initializing
    let square1 = Rectangle::square(20);
    println!(
        "The area of the square is {} square pixels.",
        square1.area()
    );
}
