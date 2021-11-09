use std::net::IpAddr;

#[derive(Debug)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            // panic! throw error
            panic!("value must be between 1 and 100, got {}", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    // throw panic if error found
    let home: IpAddr = "127.0.0.1".parse().unwrap();

    println!("home ip address: {}", home);

    let guess0 = Guess::new(20);
    dbg!(guess0.value());

    // cause error because > 100
    let guess1 = Guess::new(110);
    dbg!(guess1.value());

    // note: always use match ok error, is better
    // loop {
    //     let guess: i32 = match guess.replace("\n", "").trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue
    //     };
    //
    //     if guess < 1 || guess > 100 {
    //         println!("Secret number will between 1 and 100");
    //         continue;
    //     }
    //
    //     match guess.cmp(&secret_number) {
    //
    //     }
    // }
}
