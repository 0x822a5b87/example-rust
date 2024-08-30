extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_num = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        // In Rust, variables are immutable by default, so we must add keyword mut to specify that guess is a mutable variable.
        // The :: syntax in the String::new() line indicates that new is an associated function of
        let mut guess = String::new();

        let size = io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(v) => {
                println!("{}!", v);
                continue
            },
        };

        println!("Read size : {}, You guessed: {}", size, guess);
        match guess.cmp(&secret_num) {
            Ordering::Less => { println!("Too small!") }
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
            Ordering::Greater => { println!("Too big!"); }
        }
    }
}
