#![allow(unused_variables, dead_code)]

//! Smart pointers, on the other hand, are data structures that act like a pointer but also have additional metadata and capabilities.

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

fn main() {
    prevent_memory_leak_by_weak()
}

///
/// # Using Box<T> to Point to Data on the Heap
///
/// The most straightforward smart pointer is a box, whose type is written Box<T>.
/// Boxes allow you to store data on the heap rather than the stack.
/// Boxes don’t have performance overhead, other than storing their data on the heap instead of on the stack.
/// You’ll use them most often in these situations:
///
/// - When you have a type whose size can’t be known at compile time, and you want to use a value of that type in a context that requires an exact size
/// - When you have a large amount of data, and you want to transfer ownership but ensure the data won’t be copied when you do so
/// - When you want to own a value, and you care only that it’s a type that implements a particular trait rather than being of a specific type
pub fn using_box_to_store_data_on_the_heap() {
    // We define the variable b to have the value of a Box that points to the value 5, which is allocated on the heap.
    let b = Box::new(5);
    println!("b = {}", b);
}

///
/// # Enabling Recursive Types with Boxes
///
/// A value of recursive type can have another value of the same type as part of itself. Recursive types pose
/// an issue because at compile time Rust needs to know how much space a type takes up. However, the nesting
/// of values of recursive types could theoretically continue infinitely, so Rust can’t know how much
/// space the value needs. Because boxes have a known size, we can enable recursive types by inserting
/// a box in the recursive type definition.
pub fn enabling_recursive_types_with_boxes() {
    // enum List {
    //     Cons(i32, List), // recursive type `List` has infinite size
    //     Nil,
    // }
    //
    // let list = List::Cons(1, List::Cons(2, List::Cons(3, List::Nil)));
    //
    // error[E0072]: recursive type `List` has infinite size
    //

    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use List::{Cons, Nil};

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

///
/// # Treating Smart Pointers Like Regular References with the Deref Trait
///
/// Implementing the Deref trait allows you to customize the behavior of the dereference operator `*`
fn following_the_pointer_to_the_value() {
    let x = 5;
    let y = &5;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

///
/// # Rc<T>
///
/// the Rust type Rc<T>, which is an abbreviation for reference counting. The Rc<T> type keeps track of the number of references
/// to a value to determine whether the value is still in use. If there are zero references to a value, the value can be
/// cleaned up without any references becoming invalid.
///
/// Cons(3) --> Cons(5) --> Cons(10) --> Nil
///              ^
///              |
/// Cons(5) _____|
pub fn sharing_ownership_without_rc() {
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use List::{Cons, Nil};

    let r = Box::new(Cons(5, Box::new(Cons(10, Box::new(Nil)))));

    // error[E0382]: use of moved value: `r`
    // let top = Cons(3, r); // value moved here
    // let bottom = Cons(5, r); // value used here after move

}

/// Rc::new constructs a new Rc<T>.
/// Rc::clone makes a clone of the `Rc` pointer.
pub fn sharing_ownership_with_rc() {
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    use List::{Cons, Nil};

    let r = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating r = strong count = [{}], weak count = [{}]",
             Rc::strong_count(&r), Rc::weak_count(&r));

    // Cons(3) --> Cons(5) --> Cons(10) --> Nil
    //              ^
    //              |
    // Cons(5) _____|
    let top = Cons(3, Rc::clone(&r));

    println!("count after creating top = strong count = [{}], weak count = [{}]",
             Rc::strong_count(&r), Rc::weak_count(&r));

    let bottom = Cons(5, Rc::clone(&r));
    println!("count after creating bottom = strong count = [{}], weak count = [{}]",
             Rc::strong_count(&r), Rc::weak_count(&r));

    drop(bottom);

    println!("count after drop bottom = strong count = [{}], weak count = [{}]",
             Rc::strong_count(&r), Rc::weak_count(&r));
}

/// # RefCell<T> and the Interior Mutability Pattern
/// Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data;
/// normally, this action is disallowed by the borrowing rules. To mutate data, the pattern uses unsafe code inside a data structure to bend
/// Rust’s usual rules that govern mutation and borrowing.
///
/// With references and Box<T>, the borrowing rules’ invariants are enforced at `compile` time.
/// With RefCell<T>, these invariants are enforced at `runtime`.
///
/// The advantages of checking the borrowing rules at compile time are that errors will be caught sooner in the
/// development process, and there is no impact on runtime performance because all the analysis is completed beforehand.
/// For those reasons, checking the borrowing rules at compile time is the best choice in the majority of cases,
/// which is why this is Rust’s default.
///
/// The advantage of checking the borrowing rules at runtime instead is that certain memory-safe scenarios
/// are then allowed, where they would’ve been disallowed by the compile-time checks. Static analysis, like
/// the Rust compiler, is inherently conservative.
/// The RefCell<T> type is useful when you’re sure your code follows the borrowing rules but the compiler is unable
/// to understand and guarantee that.
///
/// Here is a recap of the reasons to choose Box<T>, Rc<T>, or RefCell<T>:
/// - Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
/// - Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.
/// - Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.
fn ref_cell_and_the_interior_mutability_pattern() {
    // struct Con {
    //     value: i32,
    //     next: Option<Rc<Con>>,
    // }
    //
    // let last = Rc::new(Con {
    //     value: 3,
    //     next: None,
    // });
    //
    //
    // let middle = Rc::new(Con {
    //     value: 2,
    //     next: Some(Rc::clone(&last)),
    // });
    //
    // let first = Rc::new(Con {
    //         value: 2,
    //         next: Some(Rc::clone(&middle)),
    //     });
    //
    // // error[E0594]: cannot assign to data in an `Rc`
    // // Rc<T> allows only immutable borrows checked at compile time;
    // last.next = Some(Rc::clone(&first));
}

fn mutate_an_immutable_value() {
    // One important part of this code is that the Messenger trait has one method called send that takes an
    // immutable reference to self and the text of the message.
    //
    pub trait Messenger {
        fn send(&self, msg: &str);
    }

    pub struct LimitTracker<'a, T: Messenger> {
        messenger: &'a T,
        value: usize,
        max: usize,
    }

    impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
    {
        pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
            LimitTracker {
                messenger,
                value: 0,
                max,
            }
        }

        pub fn set_value(&mut self, value: usize) {
            self.value = value;

            let percentage_of_max = self.value as f64 / self.max as f64;

            if percentage_of_max >= 1.0 {
                self.messenger.send("Error: You are over your quota!");
            } else if percentage_of_max >= 0.9 {
                self.messenger
                    .send("Urgent warning: You've used up over 90% of your quota!");
            } else if percentage_of_max >= 0.75 {
                self.messenger
                    .send("Warning: You've used up over 75% of your quota!");
            }
        }
    }

    // struct MockMessenger {
    //     sent_messages: Vec<String>,
    // }
    //
    // impl MockMessenger {
    //     fn new() -> MockMessenger {
    //         MockMessenger {
    //             sent_messages: vec![],
    //         }
    //     }
    // }
    //
    // impl Messenger for MockMessenger {
    //     fn send(&self, message: &str) {
    //         // 1. We can’t modify the MockMessenger to keep track of the messages,
    //         //      because the send method takes an immutable reference to self.
    //         // 2. We also can’t take the suggestion from the error text to use &mut
    //         //      self instead, because then the signature of send wouldn’t match
    //         //      the signature in the Messenger trait definition
    //         // 3. How about use &mut self in both impl and trait? This approach introduce another
    //         //      problem -- cannot borrow `*self.messenger` as mutable, as it is behind a `&` reference
    //         //
    //         // The solution is :
    //         // 1. use &mut in trait;
    //         // 2. use &mut in impl;
    //         // 3. use &mut in LimitTracker::new;
    //         // 4. use &mut in create LimitTracker.
    //         self.sent_messages.push(String::from(message));
    //     }
    // }
    //
    // let mock_messenger = MockMessenger::new();
    // let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
    //
    // limit_tracker.set_value(80);
    //
    // assert_eq!(mock_messenger.sent_messages.len(), 1);

    /// # use RefCell to void modifying & to &mut
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    let mock_messenger = MockMessenger::new();
    mock_messenger.sent_messages.borrow_mut().push(String::from("Hello!"));
    assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
}

fn ref_cell_multiple_mut_borrowing() {
    let sent_messages: RefCell<Vec<String>> = RefCell::new(vec![]);

    sent_messages.borrow_mut().push(String::from("one"));
    assert_eq!(sent_messages.borrow().len(), 1);

    sent_messages.borrow_mut().push(String::from("two"));
    assert_eq!(sent_messages.borrow().len(), 2);

    let mut one = sent_messages.borrow_mut();
    // thread 'main' panicked at src/main.rs:298:33:
    // already borrowed: BorrowMutError
    let mut two = sent_messages.borrow_mut();

    one.push(String::from("one"));
    two.push(String::from("two"));
}

fn combine_rc_and_ref_cell() {
    #[derive(Debug)]
    enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }

    use List::{Cons, Nil};

    // value = Rc(RefCell(5))
    // a = Rc(value, Nil)
    // b = Cons(Rc(RefCell(3)), a)
    // c = Cons(Rc(RefCell(4)), a)

    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
    // a after = Cons(RefCell { value: 15 }, Nil)
    // b after = Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))
    // c after = Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil))
}

fn memory_leak_by_reference_cycles() {
    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }

    use List::{Cons, Nil};

    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                Cons(_, item) => Some(item),
                Nil => None,
            }
        }
    }

    // 5
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a)); // 1
    println!("a next item = {:?}", a.tail()); // Nil

    // 10 -> 5
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a)); // 2
    println!("b initial rc count = {}", Rc::strong_count(&b)); // 1
    println!("b next item = {:?}", b.tail()); // 5

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // thread 'main' has overflowed its stack
    // fatal runtime error: stack overflow
    // println!("a next item = {:?}", a.tail());
}

fn prevent_memory_leak_by_weak() {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

fn using_box_like_a_reference() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    println!("x = {}, y = {}", x, y);
}

// Deref coercion converts a reference to a type that implements the Deref trait into a reference to another type.
fn deref_coercion_with_func_and_methods() {
    fn hello(name: &str) {
        println!("Hello, {name}!");
    }

    // &MyBoxWithTuple<String> return type &String :
    // Then deref coercion can convert &String to &str because String implements the Deref trait such that it returns &str.
    let m = MyBoxWithTuple::new(String::from("Rust"));
    let x: &MyBoxWithTuple<String> = &m;
    // this works
    hello(x);
    // this works too
    let x: &str = &m;
    hello(x);


    let rust = String::from("Rust");
    hello(&rust);
}

fn defining_our_own_smart_pointer() {
    let x = 5;
    let boxed_x = MyBoxWithTuple::new(x);

    assert_eq!(5, x);
    // access tuple in struct with index 0
    assert_eq!(5, boxed_x.0);
    assert_eq!(5, *boxed_x);

    let x = 5;
    let boxed_x = MyBoxNormal::new(x);

    assert_eq!(5, x);
    // access field in struct with field name
    assert_eq!(5, boxed_x.val);
}

// struct MyBox<T>(T); uses tuple struct syntax where the type of the field is directly specified within parentheses.
// struct MyBox<T> { val: T } uses regular struct syntax where the field name (val) and its type are explicitly specified within the struct definition.
struct MyBoxWithTuple<T>(T);
struct MyBoxNormal<T> {
    val: T,
}

impl<T> MyBoxWithTuple<T> {
    fn new(x: T) -> MyBoxWithTuple<T> {
        MyBoxWithTuple(x)
    }
}

impl<T> Deref for MyBoxWithTuple<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> MyBoxNormal<T> {
    fn new(x: T) -> MyBoxNormal<T> {
        MyBoxNormal { val: x }
    }
}

