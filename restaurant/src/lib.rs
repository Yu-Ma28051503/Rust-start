use std::fmt::Result;
use std::io::Result as IoResult;

// use std::cmp::Ordering;
// use std::io
use std::{cmp::Ordering, io};

// use std::io;
// use std::io:Write;
use std::io::{self, Write};

use std::collections::*;  // testの際に使われる

mod front_of_house;
pub use crate::front_of_house::hosting;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}



fn serve_order() {}

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

// use self::front_of_house::hosting;  // こっちでもok

pub use crate::back_of_house::Breakfast;

pub fn eat_at_restaurant() {
    // 絶対パスで呼び出し
    //crate::front_of_house::hosting::add_to_waitlist();

    // 相対パスで呼び出し
    //front_of_house::hosting::add_to_waitlist();

    // 夏にライ麦パン付き朝食を注文
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // ライ麦パンから別のパンにする
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();
}

