/*
generic type in rust dun slow code down like python
 */

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    };

    largest
}
fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// generic type function
// todo: still not work for char
// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//
//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// method only work when generic type is float
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// difference generic type
#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U
}

impl<T, U> Point2 <T, U> {
    fn mix_up<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y
        }
    }
}

// enum generic type
enum Option<T> {
    Some(T),
    None,
}

// enum done and error
enum Result<T, E> {
    Ok(T),
    Err(E)
}

// option type
#[derive(Debug)]
enum OptionI32 {
    Some(i32),
    None
}

#[derive(Debug)]
enum OptionF64 {
    Some(f64),
    None
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let number_list2 = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result2 = largest_i32(&number_list2);
    println!("The largest number is {}", result2);

    // println!("Hello, world!");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result3 = largest_char(&char_list);
    println!("The largest char is {}", result3);

    // let result_i32 = largest(&number_list);
    // let result_char = largest(&char_list);
    // println!("largest number: {}, largest char: {}", result_i32, result_char);

    let integer = Point {x: 5, y: 10 };
    let float = Point {x: 1.0, y: 4.0};
    dbg!(integer, float);

    // cause error, diff type T
    // let wont_work = Point { x: 5, y: 4.0 };

    // work for both type difference
    let both_type = Point2 { x: 5, y: 4.0 };
    dbg!(both_type);

    // method for point struct
    let p = Point {x: 5, y:10};
    println!("p.x = {}", p.x());

    let f = Point {x: 2.0, y: 4.0};
    println!("Distance from origin: {}", f.distance_from_origin());

    // point2 work diff type together
    let p1 = Point2 {x: 5, y: 10.4};
    let p2 = Point2 {x: "Hello", y: 'c'};
    let p3 = p1.mix_up(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // option type
    let integer = OptionI32::Some(5);
    let float = OptionF64::Some(5.0);
    dbg!(integer, float);
}
