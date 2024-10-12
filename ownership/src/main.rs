#![allow(dead_code)]

fn main() {

}

fn another_example() {
    let mut s = String::from("hello world");
    let slice = first_word_through_slice(&s);
    println!("first word through slice: [{}]", slice);
    println!("first word through slice: [{}]", s.len());
    // legal! because len(&self) is a immutable reference, the ownership of s doesn't change.
    println!("first word through slice: [{}]", slice);

    // clear(&mut self) needs to get a mutable reference, the ownership change.
    s.clear();
    // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    // println!("first word through slice: [{}]", slice);
}

fn first_word_through_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    // enumerate method returns a tuple, we can use patterns to destructure that tuple.
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn dangling_reference() {
    // error[E0106]: missing lifetime specifier
    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //
    //     &s
    // }
}

fn borrowing_with_mutability() {
    let mut s = String::from("hello");

    let s1 = &s;
    let s2 = &s;
    println!("s1: {}, s2: {}", s1, s2);

    // &mut will take the ownership of s away
    let mut_s = &mut s;

    // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    // println!("s1: {}, s2: {}", s1, s2);

    println!("mut_s: {}", mut_s);

    // cannot borrow `s` as mutable because it is also borrowed as immutable
    // println!("s1: {}, s2: {}, mut_s = {}", s1, s2, mut_s);
}

fn multiple_borrowing_at_once() {
    // let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;

    // cannot borrow `s` as mutable more than once at a time
    // println!("r1 = {r1}, r2 = {r2}");

    // cannot borrow `s` as mutable more than once at a time
    // println!("r1 = {r1}");
    // println!("r2 = {r2}");
}

fn modify_borrowing() {
    fn change(mut something: String) {
        something.push_str(" world");
        println!("change : something = {something}");
    }
    let hello = String::from("hello");
    println!("hello = {hello}");
    change(hello);

    // Cannot borrow immutable local variable `something` as mutable
    // fn change_with_reference(something: &String) {
    //     something.push_str(" world");
    //     println!("change_with_reference : something = {something}");
    // }

    fn change_with_mutable_reference(something: &mut String) {
        something.push_str(" world");
        println!("change_with_mutable_reference : something = {something}")
    }

    // mutable String
    let mut s = String::from("hello");
    // mutable &String
    let s = &mut s;
    change_with_mutable_reference(s)
}

fn from_string() {
    let mut s = String::from("hello");
    println!("{s}");
    s.push_str(" world");
    println!("{s}");

    let s1 = String::from("immutable hello");
    println!("s1 = {s1}");

    let mut s2 = s1;
    // ERROR : Value used after being moved [E0382]
    // println!("s1 = {s1}, s2 = {s2}");
    s2.push_str(" world");
    // ERROR : Value used after being moved [E0382]
    // println!("s1 = {s1}, s2 = {s2}");
}

fn calculate_length_with_reference() {
    fn calculate_length_with_takes(s: String) -> usize {
        s.len()
    }

    fn calculate_length_with_references(s: &String) -> usize {
        s.len()
    }

    // let s1 = String::from("hello");
    // let len = calculate_length_with_takes(s1);
    // // ERROR : Value used after being moved [E0382]
    // println!("The length of '{}' is {}.", s1, len);

    let s1 = String::from("hello");
    // reference does not take away the ownership
    // The &s1 syntax lets us create a reference that refers to the value of s1
    // but does not **OWN** it. Because it does not own it, the value it points to
    // will not be dropped when the reference stops being used.
    //
    // borrowing : We call the action of creating a reference borrowing.
    let len = calculate_length_with_references(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn return_values_and_scope() {
    fn gives_ownership() -> String {
        let some_string = String::from("yours");
        some_string
    }

    // takes ownership of a_string and gives back, the ownership is transfer from a_string to return string
    fn takes_and_gives_back(a_string: String) -> String {
        a_string
    }

    let s1 = gives_ownership();
    println!("s1 = {s1}");
    let s2 = String::from("hello");
    println!("s1 = {s1}, s2 = {s2}");
    let s3 = takes_and_gives_back(s2);
    println!("s1 = {s1}, s3 = {s3}");
}

fn test_takes_ownership_and_makes_copy() {
    let s = String::from("hello");
    takes_ownership(s);
    // ERROR : Value used after being moved [E0382]
    // println!("{s}");
    let x = 5;
    makes_copy(x);

    fn takes_ownership(some_string: String) {
        println!("{some_string}");
    }

    fn makes_copy(some_integer: i32) {
        println!("{some_integer}");
    }
}

