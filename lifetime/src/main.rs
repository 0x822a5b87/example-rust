#![allow(unused_variables, dead_code)]

fn main() {
    config_with_vec_reference()
}

fn test_copy_and_borrow() {
    fn pass_number_by_reference(number: &i8) -> bool {
        number.is_negative()
    }

    fn pass_number_by_value(number: i8) -> bool {
        number.is_negative()
    }

    fn pass_vec_by_reference(vec: &Vec<i8>) -> bool {
        vec.is_empty()
    }

    // numbers implement Copy, and so can be passed by value or reference
    let number = 42;

    // does not move number because of borrow
    let is_negative_by_ref = pass_number_by_reference(&number);

    // moves number, which can never be used again
    let is_negative_by_value = pass_number_by_value(number);
    println!("{number}");

    // copy not implemented - must be passed by reference
    let vec = vec![];

    // does not move vec
    let is_empty = pass_vec_by_reference(&vec);

    println!("is_negative_by_value: {}", is_negative_by_value);
    println!("is_negative_by_ref: {}", is_negative_by_ref);
    println!("vec {:?} is_empty: {}", vec, is_empty);
}

fn is_borrowing_work_for_all_elements() {
    #[derive(Debug)]
    struct WithCopy {
        id: i32,
    }

    impl Clone for WithCopy {
        fn clone(&self) -> Self {
            println!("copy struct [{}]", self.id);
            WithCopy { id: self.id }
        }
    }
    impl Copy for WithCopy {}

    {
        let arr = vec![
            &WithCopy { id: 1 },
            &WithCopy { id: 2 },
            &WithCopy { id: 3 },
        ];
        fn borrow_vec(arr: Vec<&WithCopy>) {}
        borrow_vec(arr);
        // no output
    }

    {
        let arr = vec![
            WithCopy { id: 1 },
            WithCopy { id: 2 },
            WithCopy { id: 3 },
        ];
        fn borrow_vec_and_elements(arr: Vec<WithCopy>) {}
        borrow_vec_and_elements(arr);
        // no output
    }
}

fn use_borrow_instead_of_move() {
    #[derive(Debug)] // no more copy
    struct Person {
        age: u8,
    }

    let alice = Person { age: 8 };
    let bob = &alice; // bob borrows alice

    println!("alice: {:?}\nbob: {:?}", alice, bob);
}

// A copy creates an exact duplicate of a value that implements the Copy trait.
// Numerical values and several other inexpensive built-in Rust types support copy.
// Vectors do not.
// Thus, struct with copy trait uses copy instead of ownership transfer.
fn distinct_between_copy_and_no_copy() {
    #[derive(Debug, Clone, Copy)]
    struct WithCopy {
        id: i32,
    }

    fn sum(val: WithCopy) {}

    let v = WithCopy { id: 0 };
    sum(v);
    println!("WithCopy.id after call = [{}]", v.id);

    struct WithoutCopy {
        id: i32,
    }
    fn sum_without_copy(val: WithoutCopy) {}
    let v = WithoutCopy { id: 0 };
    sum_without_copy(v);
    // println!("WithCopy.id after call = [{}]", v.id);
    // ERROR:
    // Value used after being moved [E0382]
}

fn test_ownership_from_return() {
    struct DropMe {
        id: u8,
    }

    impl Drop for DropMe {
        fn drop(&mut self) {
            println!("Drop!");
        }
    }

    fn return_with_ownership() -> DropMe {
        DropMe { id: 42 }
    }

    {
        println!("before call return_with_ownership() without let");
        return_with_ownership();
        println!("after call return_with_ownership() without let");
        // before call return_with_ownership
        // Drop!
        // after call return_with_ownership
        //
        // without let statement, drop() will be immediately called after function return
    }

    {
        println!("before call return_with_ownership");
        let me = return_with_ownership();
        println!("after call return_with_ownership");
        // before call return_with_ownership
        // after call return_with_ownership
        // Drop!
        // with let statement, drop() will be called after the end of the block
        // which means the lifetime is extended.
    }

    {
        println!("before call return_without_ownership");
        let drop_me = DropMe { id: 42 };
        let return_without_ownership = |d: DropMe| {
            d
        };
        return_without_ownership(drop_me);
        println!("after call return_without_ownership");
        // before call return_without_ownership
        // Drop!
        // after call return_without_ownership
    }
}

fn create_series_return_ownership() {
    fn create_series(x: i32) -> Vec<i32> {
        let result = vec![x, x + 1, x + 2];
        result
    }

    let series = create_series(42);

    println!("series: {:?}", series);
}

fn struct_drop() {
    #[derive(Debug)]
    struct DropMe;

    // Executes the destructor for this type.
    impl Drop for DropMe {
        fn drop(&mut self) {
            println!("Dropping!");
        }
    }

    println!("Begin outer scope.");

    {
        println!("Begin inner scope.");

        let x = DropMe;

        println!("x: {:?}", x);
    }

    println!("End outer scope.");
}

fn ownership_of_vec() {
    // test ownership of Vec<String>
}

fn config_with_vec_reference() {
    struct Person {
        first_name: String,
        last_name: String,
    }

    impl Person {
        pub fn new(list: Vec<String>) -> Person {
            //
            // Implicitly moving out of a Vec is not allowed as it would leave it in an invalid state — one element is moved out, the others are not.
            //
            // let x = list[0];

            let mut i = 0;
            let mut first_name = String::new();
            let mut last_name = String::new();
            for x in list {
                if i == 0 {
                    first_name = x;
                } else if i == 1 {
                    last_name = x;
                }
                i += 1;
            }

            Person { first_name, last_name }
        }
    }

    let first = String::from("hello");
    let last = String::from("world");
    let list = vec![first, last];
    let p = Person::new(list);
    println!("first = {}, last = {}", p.first_name, p.last_name);
}

fn config_with_lifetime() {
    struct Person<'a> {
        first_name: &'a String,
        last_name: &'a String,
    }

    impl<'a> Person<'a> {
        pub fn new(list: Vec<&'a String>) -> Person<'a> {
            let first: &String = list.get(0).unwrap();
            let last: &String = list.get(1).unwrap();
            Person {
                first_name: first,
                last_name: last,
            }
        }
    }

    let first = String::from("hello");
    let last = String::from("world");
    let list = vec![&first, &last];
    let p = Person::new(list);
    println!("first = {}, last = {}", p.first_name, p.last_name);
    //
    // borrowed value does not live long enough
    //
    // let p: Person;
    // {
    //     let first = String::from("hello");
    //     let last = String::from("world");
    //     let list = vec![&first, &last];
    //     p = Person::new(list);
    // }
    // println!("first = {}, last = {}", p.first_name, p.last_name);
}
