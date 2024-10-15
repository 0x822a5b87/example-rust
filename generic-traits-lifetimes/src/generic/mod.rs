#![allow(unused_variables, dead_code)]

// Rust accomplishes this by performing monomorphization of the code using generics at compile time.
// Monomorphization is the process of turning generic code into specific code by filling in the concrete types
// that are used when compiled. In this process, the compiler does the opposite of the steps we used to create
// the generic function in Listing 10-5:
// the compiler looks at all the places where generic code is called and generates code for the
// concrete types the generic code is called with.

pub fn generic_in_method_definitions_mixture() {
    struct Point<X1, Y1> {
        x: X1,
        y: Y1,
    }

    // X1 and Y1 is the generic parameter of struct Point
    impl<X1, Y1> Point<X1, Y1> {
        // X2 and Y2 is the generic parameter of function mixture
        fn mixture<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
            Point{
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixture(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

pub fn generic_in_method_definitions() {
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        pub fn x(&self) -> &T { &self.x }
        pub fn y(&self) -> &T { &self.y }
    }

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}, p.y = {}", p.x(), p.y());
}

pub fn generic_in_struct_definitions() {
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 1.0 };

    struct TwoPoint<T, F> {
        x: T,
        y: F,
    }

    let integer_and_float_point = TwoPoint { x: 5, y: 10.0 };
}

// To parameterize the types in a new single function, we need to name the type parameter,
// just as we do for the value parameters to a function. You can use any identifier as a type parameter name.
// But we’ll use `T` because, by convention, type parameter names in Rust are short, often just one letter,
// and Rust’s `type-naming convention is UpperCamelCase`. Short for type, T is the default choice of most Rust programmers.
//
// To define the generic largest function, we place type name declarations inside angle brackets, <>,
// between the name of the function and the parameter list,
pub fn generic_largest<T: PartialOrd>(list: &[T]) -> Option<&T> {
    let mut max = list.get(0)?;
    for x in list {
        if x > max {
            max = x;
        }
    }
    Some(max)
}

pub fn largest(list: &[i32]) -> Option<&i32> {
    let mut max = list.get(0)?;
    for num in list {
        if num > max {
            max = num;
        }
    }
    Some(max)
}
