// use std::fmt;
// use std::io;

use std::fmt::Result;
use std::io::Result as IoResult;

// use hashmap crate dependencies
use std::collections::HashMap;

// use crate dependencies on Cargo.toml
use rand::Rng;

// important: multi import dependencies in 1 line
use std::{cmp::Ordering, fmt};

// important: import self and child module
use std::io::{self, Write};

// important: import all module
use std::collections::*;

// use own created module
mod front_of_house;
pub use self::front_of_house::hosting::add_to_wait_list;

// use self::eat_at_restaurant;

// fix return type
// fn function1() -> fmt::Result {}
// fn function2() -> io::Result<()> {}

// fn function1() -> Result {}
// fn function2() -> IoResult<()> {}

// use lib module
use use_import::eat_at_restaurant;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    dbg!(map);

    let secret_number = rand::thread_rng().gen_range(1..101);
    dbg!(secret_number);

    add_to_wait_list();
    add_to_wait_list();
    add_to_wait_list();

    eat_at_restaurant();
}
