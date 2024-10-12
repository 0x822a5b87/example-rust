#![allow(dead_code)]

use rectangle::Rectangle;

mod rectangle;

fn main() {
    associated_function()
}

fn associated_function() {
    let rectangle    = Rectangle::new(30, 50);
    println!("associated_function");
    println!("rect1 is {rectangle:?}");
}

fn call_fn_on_struct() {
    let rect1 = &Rectangle {
        width: 30,
        height: 50,
    };

    println!("call fn on struct");
    println!("rect1 is {rect1:?}");

    println!("rect1 is {rect1:#?}");

    let rect2 = &Rectangle {
        width: 40,
        height: 20,
    };
    println!("{rect1:?} can hold {rect2:?} --> {}", rect1.can_hold(&rect2));

    let rect3 = &Rectangle {
        width: 20,
        height: 20,
    };

    println!("{rect1:?} can hold {rect3:?} --> {}", rect1.can_hold(&rect3));
}

fn adding_useful_functionality_with_derived_traits() {
    // Rust does include functionality to print out debugging information,
    // but we have to explicitly opt in to make that functionality available for our struct.
    // To do that, we add the outer attribute #[derive(Debug)] just before the struct definition,
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect1 = &Rectangle { width: 30, height: 50 };
    // rect1 is Rectangle { width: 30, height: 50 }
    println!("rect1 is {rect1:?}");

    // when we have larger struct, it's useful to have output that's a bit easier to read;
    // in those case, we can use {:#?} instead of {:?}.
    //
    // rect1 is Rectangle {
    //     width: 30,
    //     height: 50,
    // }
    println!("rect1 is {rect1:#?}");

    // 1. dbg! macro prints to the standard error console stream.
    // 2. dbg! macro takes ownership of an expression(as opposed to println!, which takes a reference)
    //
    // [src/main.rs:32:5] rect1 = Rectangle {
    //     width: 30,
    //     height: 50,
    // }
    dbg!(rect1);
}

fn refactoring_with_structure() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    fn area_structures(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

    let rect = &Rectangle { width: 30, height: 50 };
    let perimeter = area_structures(rect);
    println!("The perimeter of width = {} and height = {} is {perimeter}", rect.width, rect.height);
}

fn refactoring_with_tuples() {
    let rect1 = (30, 50);

    fn area_tuple(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }

    println!("The area is: {}", area_tuple(rect1));
}

fn unit_type_structure() {
    // These are called unit-like structs because they behave similarly to ()
    // Unit-like structs can be useful when you need to implement a trait on some type
    // but don’t have any data that you want to store in the type itself.
    struct AlwaysEqual;
    let _subject = AlwaysEqual;
}

fn create_struct_without_named_fields() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let mut _color = Color(0, 0, 0);
    let _point = Point(0, 0, 0);

    _color.0 = 1;

    println!("====create_struct_without_named_fields====");
    println!("_color.0 = {}", _color.0);
}

fn create_struct_with_dot_dot() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let _user1: User = User {
        active: false,
        username: String::from("someone"),
        email: String::from("someone@123.com"),
        sign_in_count: 1,
    };

    let _user2 = User { .._user1 };

    let _user3 = User {
        email: String::from("another_one@123.com"),
        .._user2
    };
}