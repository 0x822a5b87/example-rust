#![allow(unused_variables, dead_code)]

mod generic;
mod traits;
mod lifetimes;

fn main() {
    lifetimes::generics_traits_lifetimes()
}

fn use_summarize() {
    use traits::{Summary, Tweet};

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

fn call_largest() {
    let arr = &[3, 45, 1, 5, 2, 6, 123, 4123, 4, 7, 5, 63];
    let o = generic::largest(arr);
    match o {
        None => println!("o is None"),
        Some(v) => println!("o is the largest number {}", v),
    }
}

fn call_largest_generic() {
    let arr: &[i32; 12] = &[3, 45, 1, 5, 2, 6, 123, 4123, 4, 7, 5, 63];
    let largest_opt = generic::generic_largest(arr);
    match largest_opt {
        None => println!("error find largest!"),
        Some(v) => println!("largest is {}", v),
    }


    let arr: &[i32] = &[];
    let largest_opt = generic::generic_largest(arr);
    match largest_opt {
        None => println!("error find largest!"),
        Some(v) => println!("largest is {}", v),
    }
}