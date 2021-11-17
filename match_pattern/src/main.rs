struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message2 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

// ignore first parameters
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

struct Point2 {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // using some x1 for match
    let x1 = Some(5);
    let y1 = 10;
    // let y1: Option<&i32> = None;

    match x1 {
        Some(50) => println!("Got 50"),
        // this will match any value if wrong also match
        Some(y1) => println!("Matched, y = {:?}", y1),
        _ => println!("Default case, x = {:?}", x1),
    }

    println!("at the end: x = {:?}, y = {:?}", x1, y1);

    // next using or |
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // next using 1..x
    let x = 5;
    match x {
        // not 1..5 is 1..=5 diff
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    // match for alphabet
    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // point destructuring match
    let p = Point { x: 0, y: 7 };
    match p {
        // y must 0
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        // x must 0
        Point { x: 0, y } => println!("On the y axis at {}", y),
        // anything
        Point { x, y} => println!("On neither axis: ({}, {})", x, y),
    }

    // next enum match
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        },
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x,
                y
            )
        },
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        },
    }

    // destruct nested enum match
    let msg2 = Message2::ChangeColor(Color::Hsv(0, 160, 255));
    match msg2 {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        },
        Message2::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h,
                s,
                v
            )
        },
        _ => ()
    }

    // destructuring struct and tuple
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("feet: {}, inches: {}, x: {}, y: {}", feet, inches, x, y);

    // ignore first parameters
    foo(1, 2);

    // ignore part of value with a nested
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        // any and any value
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        },
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    // ignore part of value with a nested
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        },
    }

    // ignore warning
    let _x = 5; // do not give warning
    let y = 10; // give warning

    // let s = Some(String::from("Hello!"));
    //
    // if let Some(_s) = s {
    //     println!("found a string");
    // }
    //
    // println!("{:?}", s);

    // ignore part of value with struct
    let origin = Point2 { x: 0, y: 0, z: 0 };
    match origin {
        // ignore all after x
        Point2 { x, .. } => println!("x is {}", x),
    }

    // ignore middle of list value
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        },
    }

    // not work like this, start need value
    // match numbers {
    //     (.., second, ..) => {
    //         println!("Some numbers: {}", second);
    //     },
    // }

    // extra condition when matched
    let num = Some(4);
    match num {
        // x < 5 to match with condition
        Some(x) if x < 5 => println!("less than five: {}", x),
        // x any value
        Some(x) => println!("{}", x),
        None => (),
    }

    // match other variables
    let x = Some(10);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // n must be equal to y
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    // match condition with or | also work
    let x = 4;
    let y = false;
    match x {
        // match any 4 5 6 and y is true
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // @ binding
    enum Message3 {
        Hello { id: i32 },
    }

    // enum using @ to match condition
    let msg = Message3::Hello { id: 5 };
    match msg {
        // id 3..7
        Message3::Hello {
            id: id_variable @ 3..=7
        } => println!("Found an id in range: {}", id_variable),
        // id 10 to 12
        Message3::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        },
        // id exists
        Message3::Hello { id } => println!("Found some other id: {}", id),
    }
}
