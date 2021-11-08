// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_wait_list() {}
//     }
// }

// use absolute path
// use crate::front_of_house::hosting;

// use relative path with self
// use self::front_of_house::hosting;

// use module
mod front_of_house;

// export module
pub use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    println!("Eat at restaurant");
    hosting::add_to_wait_list();
    hosting::add_to_wait_list();
    hosting::add_to_wait_list();
}

// use function inside module
use crate::front_of_house::hosting::add_to_wait_list;

pub fn eat_at_restaurant2() {
    add_to_wait_list();
    add_to_wait_list();
    add_to_wait_list();
}

