// lifetimes ensure that references are valid as long as we need them to be

// Rust uses a borrow checker to determine that this code is invalid
pub fn preventing_dangling_references_with_lifetimes() {
    // let r;
    // {
    //     let x = 5;
    //     // error[E0597]: `x` does not live long enough
    //     r = &x;
    // }
    // println!("r: {}", r);
}

pub fn borrow_checker_references() {
    // Here, we’ve annotated the lifetime of r with 'a and the lifetime of x with 'b.
    // As you can see, the inner 'b block is much smaller than the outer 'a lifetime block.
    // At compile time, Rust compares the size of the two lifetimes and sees that r has a lifetime of 'a
    // but that it refers to memory with a lifetime of 'b.
    // The program is rejected because 'b is shorter than 'a: the subject of the reference doesn’t live as long as the reference.
    //
    // fn borrow_checker_1() {
    //     let r;            // ---------+-- 'a
    //     //                                 |
    //     {                      //          |
    //         let x = 5;        // -+-- 'b   |
    //         r = &x;           //  |        |
    //     }                     // -+        |
    //     //                                 |
    //     println!("r: {r}");   //           |
    // }                         //  ---------+


    // Here, x has the lifetime 'b, which in this case is larger than 'a.
    // This means r can reference x because Rust knows that the reference in r will always be valid while x is valid.
    //
    //
    // let x = 5;            // ----------+-- 'b
    // //                                 |
    // let r = &x;           // --+-- 'a  |
    // //                         |       |
    // println!("r: {r}");   //   |       |
    // //                       --+       |
}

// Remember, when we specify the lifetime parameters in this function signature,
// we’re not changing the lifetimes of any values passed in or returned.
// Rather, we’re specifying that the borrow checker should reject any values that don’t adhere to these constraints.
pub fn generic_lifetimes_in_functions() {
    // Lifetime annotations don’t change how long any of the references live.
    // Rather, they describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.
    //
    // Lifetime annotations have a slightly unusual syntax: the names of lifetime parameters must start
    // with an apostrophe (') and are usually all lowercase and very short, like generic types.
    //
    //
    // &i32
    // &'a i32
    // &'a mut i32
    //
    // We want the signature to express the following constraint:
    // the returned reference will be valid as long as both the parameters are valid.
    // This is the relationship between lifetimes of the parameters and the return value.
    // We’ll name the lifetime 'a and then add it to each reference
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }

    let string1 = String::from("hello");
    let string2 = " world";

    // Note that we want the function to take string slices, which are references, rather than strings,
    // because we don’t want the longest function to take ownership of its parameters.
    let result = longest(string1.as_str(), string2);
    println!("The longest string is [{}]", result);
}

pub fn different_generic_lifetimes_in_functions() {
    fn longest<'b>(x: &'b str, y: &'b str) -> &'b str {
        if x.len() > y.len() { x } else { y }
    }

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }
    // at this point, string2 and result is invalid, nevertheless string1 is still valid
    // which means the function `longest` must have different concrete lifetimes.
}
// string1 is valid until the end of this function.
//
//

// pub fn another_different_generic_lifetimes_in_functions() {
//     fn longest<'c>(x: &'c str, y: &'c str) -> &'c str {
//         if x.len() > y.len() { x } else { y }
//     }
//
//     let string1 = String::from("long string is long");
//     let result;
//     {
//         let string2 = String::from("xyz");
//         result = longest(string1.as_str(), string2.as_str());
//     }
//     // at this point string2 is invalid, meanwhile result and string1 is still valid.
//     // the problem is result may have a reference to string2, which indicates string2 would need to be
//     // valid until the end of the outer scope. Otherwise, the result in the outer scope will be invalid too.
//     println!("The longest string is {result}");
// }

//
// The problem is that result goes out of scope and gets cleaned up at the end of the longest function.
// We’re also trying to return a reference to result from the function. There is no way we can specify lifetime parameters
// that would change the dangling reference, and Rust won’t let us create a dangling reference.
//
// pub fn lifetime_only_in_return_value_and_function() {
//     // error[E0515]: cannot return value referencing local variable `s`
//     fn largest<'a> (x: &str, y: &str) -> &'a str {
//         let s = String::from("lifetime_only_in_return_value_and_function");
//         s.as_str()
//     }
// }

pub fn lifetime_annotation_in_struct() {
    // In this statement, we declare a struct with lifetime 'a, and it contains a member called part with lifetime 'a also.
    // And the lifetime indicates that the member part must have a lifetime as longer as the lifetime of struct at least.
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("important excerpt is [{}]", i.part);
}

pub fn lifetime_elision() {
    // lifetime elision
    //
    // The first rule is that the compiler assigns a lifetime parameter to each `parameter` that’s a reference.
    // In other words, a function with one parameter gets one lifetime parameter:
    //
    // fn foo<'a>(x: &'a i32);
    //
    // a function with two parameters gets two separate lifetime parameters:
    //
    // fn foo<'a, 'b>(x: &'a i32, y: &'b i32);
    //
    // and so on.

    // The second rule is that, if there is exactly one input lifetime parameter,
    // that lifetime is assigned to all output lifetime parameters:
    //
    // fn foo<'a>(x: &'a i32) -> &'a i32.

    // The third rule is that, if there are multiple input lifetime parameters, but one of them
    // is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters.
    // This third rule makes methods much nicer to read and write because fewer symbols are necessary.
}

pub fn static_lifetimes() {
    // One special lifetime we need to discuss is 'static, which denotes that the affected reference can live for the
    // entire duration of the program. All string literals have the 'static lifetime, which we can annotate as follows:
    let s: &'static str = "I have a static lifetime.";
    println!("static_lifetimes s is {}", s);
}

pub fn generics_traits_lifetimes() {
    use std::fmt::Display;
    use std::fmt;

    struct Position {
        longitude: f32,
        latitude: f32,
    }

    impl Display for Position {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.longitude, self.latitude)
        }
    }

    fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, announcement: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", announcement);

        if x > y { x } else { y }
    }

    let x = "hello world";
    let y = "this is my first generics statement!";
    let position = Position { longitude: 1.0, latitude: 2.0 };
    let longest = longest_with_announcement(x, y, position);
    println!("longest: {}", longest);
}