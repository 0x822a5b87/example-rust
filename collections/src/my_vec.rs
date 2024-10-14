#![allow(unused_variables, dead_code)]

pub fn using_an_enum_to_store_multiple_types() {
    pub enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(1),
        SpreadSheetCell::Float(10.0),
        SpreadSheetCell::Text(String::from("blue")),
    ];

    for i in &row {
        match i {
            SpreadSheetCell::Int(int) => println!("int : {}", int),
            SpreadSheetCell::Float(float) => println!("float : {}", float),
            SpreadSheetCell::Text(text) => println!("text : {}", text),
        }
    }
}

pub fn iterating_over_the_values_in_vec() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50
    }

    for i in &v {
        println!("{i}");
    }
}

pub fn reading_element_after_push() {
    let mut v = vec![1, 2, 3];

    let third = &v[2]; // immutable borrow occurs here

    // error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
    v.push(4); // mutable borrow occurs here

    // println!("third element is {}", third); // immutable borrow later used here
}

pub fn reading_elements_of_vectors() {
    let v = vec![1, 2, 3, 4, 5];

    let third = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        None => println!("The third element is None"),
        Some(v) => println!("The third element is {}", v),
    }

    let does_not_exist = v.get(100);
    match does_not_exist {
        None => println!("The 100th element is None"),
        Some(v) => println!("The 100th element is {}", v),
    }
}

pub fn new_vec() {
    let _v: Vec<i32> = Vec::new();
    let _v = vec![1, 2, 3];

    let mut v = vec![1, 2, 3];

    v.push(4);
    v.push(5);
    v.push(6);
}
