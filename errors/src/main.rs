use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
}

pub fn last_char_of_first_line(text: &str) -> Option<char> {
    // ? can be used with Option<T> values as well
    // The behavior of the ? operator when called on an Option<T> is similar to its behavior
    // when called on a Result<T, E>:
    // if the value is None, the None will be returned early from the function at that point.
    // If the value is Some, the value inside the Some() is the resultant value of the expression, and the function continues.
    text.lines().next()?.chars().last()
}

pub fn propagating_errors_with_question_mark2(filename: &String) -> Result<String, io::Error> {
    let mut username = String::new();
    File::open(filename)?.read_to_string(&mut username)?;
    Ok(username)
}

pub fn propagating_errors_with_question_mark(filename: &String) -> Result<String, io::Error> {
    // If the value of the Result is an Ok, the value inside the Ok will
    // get returned from this expression, and the program will continue.
    // If the value is an Err, the Err will be returned from the whole function as if
    // we had used the return keyword so the error value gets propagated to the calling code.
    //
    // the ? operator is defined to perform an early return of a value out of the function,
    let mut username_file = File::open(filename)?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    println!("File name: {}", username);
    Ok(username)
}

pub fn propagating_errors() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

pub fn match_on_different_error() {
    let greeting_file_result = File::open("hello.txt");

    File::open("xxx.txt").expect("Something went wrong");

    let greeting_file = greeting_file_result.unwrap_or_else(|error| match error.kind() {
        ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Problem creating the file: {e:?}"),
        },
        other_error => {
            panic!("Problem opening the file: {other_error:?}");
        }
    });
    println!("file = {greeting_file:?}")
}

pub fn open_with_with_error_handling() {
    let greeting_file_result = File::open("hello.txt");

    match &greeting_file_result {
        Ok(v) => println!("File successfully opened: {:?}", v),
        Err(e) => println!("Error: {:?}", e.to_string()),
    }


    let greeting_file_result = File::open("main.rs");
    let file = match greeting_file_result {
        Ok(v) => v,
        Err(e) => panic!("Problem opening the file : {e:?}"),
    };
    println!("file = {:?}", file);
}