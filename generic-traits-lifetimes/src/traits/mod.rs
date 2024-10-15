#![allow(unused_variables, dead_code)]

use std::fmt::{Debug, Display};

// A trait defines the functionality a particular type has and can share with other types.
// We can use traits to define shared behavior in an abstract way.
// We can use trait bounds to specify that a generic type can be any type that has certain behavior
//
// Note: Traits are similar to a feature often called interfaces in other languages, although with some differences.
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }

    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// trait as parameter
// The impl Trait syntax is convenient and makes for more concise code in simple cases,
// while the fuller trait bound syntax can express more complexity in other cases.
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// trait bound generic type
pub fn notify_generic<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn compare_impl_trait_notify_and_trait_bound_notify() {
    // Using impl Trait is appropriate if we want this function to allow item1 and item2 to
    // have different types (as long as both types implement Summary).
    // If we want to force both parameters to have the same type, however, we must use a trait bound, like this:
    pub fn impl_trait_notify(item1: &impl Summary, item2: &impl Summary) {}
    pub fn trait_bound_notify<T: Summary>(item1: &T, item2: &T) {}
}

pub fn specifying_multiple_trait_bound() {
    pub fn impl_trait_notify(item: &(impl Summary + Display)) {}
    pub fn trait_bound_notify<T: Summary + Display>(item: &T) {}
}

pub fn clearer_trait_with_where() {
    fn some_function_with_trait<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 { 0 }
    fn some_function<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    { 0 }
}

pub fn returns_summary() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// However, you can only use impl Trait if you’re returning a single type.
// For example, this code that returns either a NewsArticle or a Tweet with the return type specified as impl Summary wouldn’t work:
// Returning either a NewsArticle or a Tweet isn’t allowed due to restrictions around how the impl Trait syntax is implemented in the compiler
//
//
// fn returns_multiple_summarize(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from(
//                 "Penguins win the Stanley Cup Championship!",
//             ),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

pub fn conditionally_implement_methods() {
    struct Pair<T> {
        x: T,
        y: T,
    }

    impl <T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    // Conditionally implementing methods on a generic type depending on trait bounds
    // It's not equivalent to partial specialization
    // The trait constraint here only restricts the behavior of type T, without specializing Pair<T>.
    // Regardless of the specific type of T, the implementation of Pair<T> is the same, except that the cmp_display() method is constrained.
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
}