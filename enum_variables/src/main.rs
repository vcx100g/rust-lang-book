// enum use for declare custom data types only
// use for match compare and condition
#[derive(Debug)]
enum IpAddressKind {
    V4,
    V6
}

#[derive(Debug)]
struct IpAddress {
    kind: IpAddressKind,
    address: String,
}

// better use enum than struct
#[derive(Debug)]
enum IpAddress2 {
    V4(String),
    V6(String)
}

// enum multi with multiple tuple data types
#[derive(Debug)]
enum IpAddress3 {
    V4(u8, u8, u8, u8),
    V6(String)
}

// multi data type enum
#[derive(Debug)]
enum Message {
    Quit, // no data type
    Move { x: i32, y: i32}, // struct
    Write(String), // string
    ChangeColor(i32, i32, i32) // tuple integer32
}

// same as above but using struct
#[derive(Debug)]
struct QuitMessage;
#[derive(Debug)]
struct MoveMessage {
    x: i32,
    y: i32
}
#[derive(Debug)]
struct WriteMessage(String);
#[derive(Debug)]
struct ChangeColorMessage(i32, i32, i32);

// enum method
impl Message {
    fn call(&self) {
        // nothing yet
    }
}

// like typescript, no default type, use T as type
// input integer will become Integer
#[derive(Debug)]
enum Option<T> {
    None,
    Some(T),
}

fn main() {
    // using enum
    let four = IpAddressKind::V4;
    let six = IpAddressKind::V6;

    // using struct
    let home = IpAddress {
        kind: IpAddressKind::V4,
        address: String::from("127.0.0.1")
    };
    let loopback = IpAddress {
        kind: IpAddressKind::V6,
        address: String::from("::1")
    };
    dbg!(home, loopback);

    // using enum type
    let home1 = IpAddress2::V4(String::from("127.0.0.1"));
    let loopback1 = IpAddress2::V6(String::from("::1"));
    // println!("home1: {:?}", home1);
    dbg!(home1, loopback1);

    // using enum with tuple variables
    let home2 = IpAddress3::V4(127,0,0,1);
    let loopback2 = IpAddress3::V6(String::from("::1"));
    dbg!(home2, loopback2);

    // message
    let m = Message::Write(String::from("hello"));
    m.call();
    dbg!(m);

    // enum with T
    let some_number = Some(5);
    let some_string = Some("a string");
    // let absent_number: Option<i32> = None;
    dbg!(some_number, some_string);

    // x and y are not same data types
    let x: i8 = 5;
    let y: Option<i8> = Option::Some(5);
    dbg!(x, y);
    // let sum = x + y;
    // dbg!(x, y);

    let message_move = Message::Move {x: 8, y: 8};
    let message_write = Message::Write(String::from("hello"));
    dbg!(message_write);
}
