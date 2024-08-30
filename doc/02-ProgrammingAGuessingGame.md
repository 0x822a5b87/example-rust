# 2 - Programming a Guessing Game

1. In Rust, variables are immutable by default, so we must add keyword `mut` to specify that guess is a mutable variable.
2. The `::` syntax in the `String::new()` line indicates that `new` is an **associated function** of `String` type.An associated function is implemented on a `type`, in this case `String`, rather than on a particular instance of a String.Some languages call this a `static method`.

```rust
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```



