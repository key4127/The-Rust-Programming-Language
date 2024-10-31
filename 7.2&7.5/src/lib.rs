pub mod front_of_house;

mod back_of_house {
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
    pub enum Appetizer {
        Soup,
        Salad,
    }

}

pub fn cook_order(toast: &str) {
    crate::front_of_house::hosting::add_to_waitlist();

    let mut meal = crate::back_of_house::Breakfast::summer(toast);
    println!("Here is your {} toast.", meal.toast);
    //println!("{}", meal.seasonal_fruit);

    let order1 = crate::back_of_house::Appetizer::Soup;
    let order2 = crate::back_of_house::Appetizer::Salad; 
}