// take 1 tuple for 2 parameters
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    // let variable pattern example
    let favorite_color: Option<&str> = None; // must set none first
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    // let some none for variables, good handling
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // while let pattern example
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    // each pop will run println!
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for loop example, basic
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // destructuring example
    let (x, y, z) = (1, 2, 3);
    println!("x: {}, y: {}, z: {}", x, y, z);

    // tuple parameters
    let point = (3, 5);
    print_coordinates(&point);
}
