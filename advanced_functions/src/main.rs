fn add_one(x: i32) -> i32 {
    x + 1
}

// function as parameters
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

// function return another function using box
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    // use closure
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    println!("{:?}", list_of_strings);

    // use builtin functions
    let list_of_strings2: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    println!("{:?}", list_of_strings2);

    // use enum map for vector
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!("{:?}", list_of_statuses);

    let add_one_closure = returns_closure();
    println!("{:?}", add_one_closure(1));
}
