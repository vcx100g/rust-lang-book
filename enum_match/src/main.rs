#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

// match enum
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // new method: None
        None => None,
        // new method: Some is contain value, must use with None
        Some(i) => Some(i + 1)
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}

fn main() {
    println!("Penny: {}", value_in_cents(Coin::Penny));
    println!("Nibble: {}", value_in_cents(Coin::Nickel));
    println!("Dime: {}", value_in_cents(Coin::Dime));
    println!("Quarter: {}", value_in_cents(Coin::Quarter(UsState::Alabama)));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    dbg!(five, six, none);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // _ => (), // run nothing
        _ => reroll()  // new method: _ placeholder like javascript
    }

    let config_max = Some(3u8);

    // if else
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // match some
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => ()
    }

    // replace if else with match enum
    let coin = Coin::Quarter(UsState::Alaska);
    let mut count = 0;

    // using if else
    if let Coin::Quarter(UsState::Alaska) = coin {
        println!("State quarter from {:?}!", UsState::Alaska)
    } else {
        count += 1;
    }

    // better than if else
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1
    }
}
