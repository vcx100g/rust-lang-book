// new method: add debug implementation to rectangle
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels. (variables)",
        area(width1, height1)
    );

    // tuple
    let rect1 = (40, 40);

    println!(
        "The area of the rectangle is {} square pixels. (tuple)",
        area2(rect1)
    );

    // struct
    let rect2 = Rectangle {
        width: 33,
        height: 55
    };

    println!(
        "The area of the rectangle is {} square pixels. (tuple)",
        area3(&rect2)
    );

    println!("rectangle: {:?}", rect2);

    // new method: dbg! debug, like console debug
    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    // output debug for rect3
    dbg!(&rect3);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// using tuple parameters
fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}