// a new module
mod front_of_house {
    // public module
    pub mod hosting {
        // public method
        pub fn add_to_wait_list() {}

        pub fn seat_at_table() {}
    }

    pub mod serving {
        pub fn take_order() {}

        pub fn serve_order() {}

        pub fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_wait_list();

    // relative path
    front_of_house::hosting::add_to_wait_list();
}

// module with struct and method
mod back_to_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }
}

pub fn eat_at_restaurant2() {
    let mut meal = back_to_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}

fn serve_order() {}

// use super in module
mod back_to_house2 {
    fn fix_incorrect_order() {
        cook_order();
        // new method: super call outside function
        super::serve_order();
    }

    fn cook_order() {}
}

// use enum in module
mod back_to_house3 {
    pub enum Appetizer {
        Soup,
        Salad
    }
}

pub fn eat_at_restaurant4() {
    let order1 = back_to_house3::Appetizer::Soup;
    let order2 = back_to_house3::Appetizer::Salad;
}