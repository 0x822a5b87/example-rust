#![allow(unused_variables, dead_code)]

pub fn iterating_over_strings() {
    // iterating over strings on chars
    for c in "Здравствуйте".chars() {
        print!("{c}")
    }
    println!();

    // iterating over strings on bytes
    for c in "Здравствуйте".bytes() {
        print!("{c} ")
    }
    println!();
}

pub fn slicing_strings() {
    let hello = "Здравствуйте";

    let s = &hello[0..4];
    println!("{}", s); // Зд

    // let s = &hello[0..3];
    // byte index 3 is not a char boundary; it is inside 'д' (bytes 2..4) of `Здравствуйте`
    // println!("{}", s);
}

pub fn indexing_into_strings() {
    let hello = &String::from("Hello");

    // error[E0277]: the type `str` cannot be indexed by `{integer}`
    // println!("Hello[0] = {}", hello[0]);
    // A String is a wrapper over a Vec<u8> which means access string through index may produce unexpected return value.
}

pub fn concatenation_with_plus_operator() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    // equivalent to uses the add method, whose signature looks something like this
    // fn add(self, s: &str) -> String {
    // that means s1's ownership is transfer
    let s3 = s1 + &s2;

    println!("s2 = {s2}, s3 = {s3}");

    // ERROR : Value used after being moved [E0382]
    // println!("s1 = {s1}, s2 = {s2}, s3 = {s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s = {}", s);
}

pub fn create_a_new_string() {
    let s = String::new();
    let s = &String::from("");

    let data: &str = "initial content";

    let s: String = data.to_string();
}