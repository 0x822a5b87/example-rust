#![allow(unused_variables, dead_code)]

pub fn updating_an_value_based_on_the_old_value() {
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}

pub fn hash_map_and_ownership() {
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    // code snippet works fine
    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);
    println!("field_name = {field_name}, field_value = {field_value}");


    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point
    // because compiler infer the map's type is HashMap<String, String> which means there is an ownership transfer
    // println!("field_name = {field_name}, field_value = {field_value}",)
}

pub fn iterating_hash_map() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    for (key, value) in scores {
        println!("{key}: {value}");
    }
}

pub fn accessing_values_in_hash_map() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");

    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("team score is {}", score);
}

pub fn creating_hash_map() {
    let mut scores = std::collections::HashMap::new();

    // This is legal
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");
    scores.insert(&blue, 10);
    scores.insert(&yellow, 50);

    // error[E0716]: temporary value dropped while borrowed
    // 10 |     scores.insert(&String::from("Blue"), 10);
    //    |                    ^^^^^^^^^^^^^^^^^^^^     - temporary value is freed at the end of this statement
    //    |                    |
    //    |                    creates a temporary value which is freed while still in use
    // 11 |     scores.insert(&String::from("Yellow"), 50);
    //    |     ------ borrow later used here
    // scores.insert(&String::from("Blue"), 10);
    // scores.insert(&String::from("Yellow"), 50);
}