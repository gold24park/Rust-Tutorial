// use std::fmt::Result;
// use std::io::Result as IoResult;

// use std::cmp::Ordering;
// use std::io;
// Nested Paths
// use std::{cmp::Ordering, io};

// use std::io;
// use std::io::Write;
use std::io::{self, Write};

// Glob Operator
// Be careful when using the glob operator!
// Glob can make it harder to tell what names are in scope and where a name used in your program was defined.
use std::collections::*;

fn deliver_order() {}

mod back_of_house {

    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

// Re-exporting
// External code would have to call add_to_waitlist() by using
// restaurant::front_of_house::hosting::add_to_waitlist()
// restaurant::hosting::add_to_waitlist()

// [?] 왜 못찾지?
mod front_of_house;

pub fn eat_at_restaurant() {
    front_of_house::hosting::add_to_waitlist();
}
