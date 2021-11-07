fn main() {
    let number = 6;

    if number < 5 {
        println!("less than 5");
    } else {
        println!("greater or equal to 5");
    }

    // only work for boolean
    // if number {
    if number != 0 {
        println!("Number is not 0");
    }

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2");
    }

    // single line condition flow
    let condition = true;
    let number = if condition { 5 } else { 6 };
    // this return diff type wont work
    // let number = if condition { 5 } else { "six" };
    println!("Number is: {}", number);

    // loop flow
    let mut count = 3;
    loop {
        println!("count: {}", count);
        count -= 1;
        if count < 1 { break }
    }

    // big loop
    let mut count = 0;
    // new code: outer loop name: counting_up
    'counting_up: loop {
        println!("count: {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining: {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                // break outer loop
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count: {}", count);

    // variables assign by loop
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Result: {}", result);

    // while
    let mut number = 3;
    while number != 0 {
        println!("while: {}", number);
        number -= 1;
    }

    // loop through a collection
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("index: {}", a[index]);
        index += 1;
    }

    // for in loop
    for element in a {
        println!("element: {}", element);
    }

    // using range of number
    // new code: range using (1..10)
    for number in (1..4).rev() {
        println!("number: {}", number);
    }
    println!("LIFTOFF!!!");
}
