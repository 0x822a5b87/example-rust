//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

use crate::art::{PrimaryColor, SecondaryColor};

mod art;

/// Adds one to the number given.
// --snip--


fn main() {
}

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(8, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/// Send http post request
///
/// # Examples
///
/// ```
/// let url = &String::from("127.0.0.1:8080/index");
/// post(url).unwrap();
/// ```
///
/// # Panics
///
/// Panics if the request timeout
///
/// # Errors
///
/// maybe a timeout error
pub fn post(_url: &String) -> Result<(), std::io::Error> {
    Ok(())
}

pub fn choose_blue() -> PrimaryColor {
    PrimaryColor::Blue
}
pub fn choose_orange() -> SecondaryColor {
    SecondaryColor::Orange
}
