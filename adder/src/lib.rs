pub fn add_two(a: i32) -> i32 { a + 2 }

pub fn greeting(name: &str) -> String {
    format!("Hello {name}")
}

// The purpose of unit tests is to test each unit of code in isolation from the rest of the code to quickly
// pinpoint where code is and isn’t working as expected. You’ll put unit tests in the src directory in each file
// with the code that they’re testing.
//
// The convention is to create a module named tests in each file to contain the test functions and to
// annotate the module with cfg(test).
// The #[cfg(test)] annotation on the tests module tells Rust to compile and run the test code only when
// you run cargo test, not when you run cargo build.
//
// the attribute cfg stands for configuration and tells Rust that the following item should only be included
// given a certain configuration option. In this case, the configuration option is test, which is provided by
// Rust for compiling and running tests.
#[cfg(test)]
mod tests {
    use std::panic;
    use crate::{add_two, greeting};

    // the test annotation is an attribute that indicates this is a test function
    #[test]
    fn larger_can_hold_smaller() {
        // The tests module is a regular module that follows the usual visibility rules.
        // Because the tests module is an inner module, we need to bring the code under test
        // in the outer module into the scope of the inner module.
        let larger = super::Rectangle { width: 8, height: 7 };
        let smaller = super::Rectangle { width: 5, height: 1 };
        assert_eq!(larger.can_hold(&smaller), true);
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = super::Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = super::Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        let result = add_two(2);
        // Note that in some languages and test frameworks, the parameters to equality assertion functions
        // are called expected and actual, and the order in which we specify the arguments matters.
        // However, in Rust, they’re called left and right, and the order in which we specify the value
        // we expect and the value the code produces doesn’t matter
        assert_eq!(result, 4);
    }

    #[test]
    fn greeting_contains_name() {
        let name = "Carol";
        let result = greeting(name);
        assert!(result.contains("Carol"),
        "Greeting did not contain name `{}`, result was `{}`", name, result);
    }

    #[test]
    #[should_panic(expected = "this test should panic!")]
    fn should_panic() {
        panic!("this test should panic!")
    }

    #[test]
    fn expect_error() {
        let result = panic::catch_unwind(|| {
            panic!("catch unwind")
        });

        assert!(result.is_err());
    }

    #[test]
    fn test_result() -> Result<(), String> {
        // In the body of the function, rather than calling the `assert_eq!` macro
        // we return `Ok(())` when the test passes and an `Err` with a String inside when the test fails.
        let ret = add_two(100);
        if ret == 102 {
            Ok(())
        } else {
            Err("should never get here".into())
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
