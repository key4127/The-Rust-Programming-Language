// 7.3

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    // struct
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

    // enum
    pub enum Appetizer {
        Soup,
        Salad,
    }
}


// 7.4

// shortcut in particular scope
use crate::front_of_house::hosting;

mod customer {
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant () {
        hosting::add_to_waitlist();
    }
}

pub fn eat_at_restaurant() {

    // 7.3

    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    println!("The initial toast is {}", meal.toast);
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // Error
    // println!("The seasonal fruit is {}", meal.seasonal_fruit)

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;


    // 7.4

    hosting::add_to_waitlist();
}

// specify parent modules to avoid duplicate name

/*
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    Ok(())
}

fn function2() -> io::Result<()> {
    Ok(())
}
*/

// provide new name

use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    Ok(())
}

fn function2() -> IoResult<()> {
    Ok(())
}

// pub use

/*
pub use crate::front_of_house::hosting;

mod customers {
    pub fn eat() {
        // not crate::front_of_house::hosting
        crate::hosting::add_to_waitlist();
    }
}
*/

// other syntax

/*
use std::{cmp::Ordering, io};
use std::io::{self, Write};
use std::collections::*;
*/