use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..100);
    let mut guess: String = String::new();

    loop {
        println!("Please input your guess 1-100:");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Please input a valid guess");

        // todo: unable to convert string to number
        // match in rust === switch in js
        let guess: u32 = match guess.replace("\n", "").trim().parse() {
            Ok(num) => num,
            Err(error) => {
                println!("error: {}", error);
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }

    // let apples = 5; // immutable
    // let mut bananas = 10; // mutable
    // bananas = 3;
    //
    // println!("apples: {}, bananas: {}", apples, bananas);
    //
    // let x = 5;
    // let y = 10;
    // println!("x: {}, y: {}", x, y);
}
