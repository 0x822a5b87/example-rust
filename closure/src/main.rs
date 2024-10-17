#![allow(unused_variables, dead_code)]

use std::thread;

fn main() {
    iterator_demonstration();
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn inventory() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}

// Closures can capture values from their environment in three ways, which directly map to
// the three ways a function can take a parameter: borrowing immutably, borrowing mutably, and taking ownership.

// we define a closure that captures an immutable reference to the vector named list
// because it only needs an immutable reference to print the value:
fn capture_an_immutable_reference() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
}

fn capture_an_mutable_reference() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {list:?}");
}

// If you want to force the closure to take ownership of the values it uses in the environment
// even though the body of the closure doesn’t strictly need ownership, you can use the move
// keyword before the parameter list.
// This technique is mostly useful when passing a closure to a new thread to move the data so that it’s owned by the new thread.
fn taking_ownership() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}

// The way a closure captures and handles values from the environment affects which traits
// the closure implements, and traits are how functions and structs can specify what kinds
// of closures they can use.
// Closures will automatically implement one, two, or all three of these Fn traits,
// in an additive fashion, depending on how the closure’s body handles the values:
//
// 1. FnOnce applies to closures that can be called once. All closures implement at least
//  this trait, because all closures can be called. A closure that moves captured values out
//  of its body will only implement FnOnce and none of the other Fn traits,
//  because it can only be called once.
//
// 2. FnMut applies to closures that don’t move captured values out of their body,
//  but that might mutate the captured values. These closures can be called more than once.
//
// 3. Fn applies to closures that don’t move captured values out of their body and that don’t
//  mutate captured values, as well as closures that capture nothing from their environment.
//  These closures can be called more than once without mutating their environment, which is
//  important in cases such as calling a closure multiple times concurrently.
fn sort_by_ket() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    println!("{list:#?}");
}

fn sorted_operations() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
}

fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    while let Some(element) = v1_iter.next() {
        println!("{}", element);
    }
}