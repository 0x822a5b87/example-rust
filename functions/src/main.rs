#![allow(unused_variables, dead_code)]

fn main() {
    loop_with_index_rev()
}

fn loop_with_index_rev() {
    for index in (0..4).rev() {
        println!("index = {index}")
    }
}

fn loop_with_index() {
    let a = [1, 2, 3, 4, 5];
    for index in 0..a.len() {
        println!("index = {index}, value = {}", a[index])
    }
}

fn loop_with_for() {
    let a = [1, 2, 3, 4, 5];
    for (index, v ) in a.into_iter().enumerate() {
        println!("index = {index}, value = {v}")
    }
}

fn loop_labels_to_disambiguate_between_multiple_loops() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");

        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1
    }

    println!("End count = {count}")
}

fn returning_values_from_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 10;
        };
    };

    println!("returning_values_from_loop: counter = {}, return = {}", counter, result)
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x)
}

fn expression_can_be_part_of_statements() {
    // 100 is an expression and is part of the statement
    let x: i32 = 100;
    println!("{}", x)
}

fn calling_a_function_is_a_expression() {
    let _r = expression_can_be_part_of_statements();
}

fn calling_a_macro_is_a_expression() {
    let _r = println!("Hello world!");
}

fn curly_bracket_is_a_expression() {
    let y = {
        let x = 3;
        // Attention: the following code is very different:
        // x + 1;
        // x + 1
        // the first line ends with semicolon, which means the evaluation result of the expression would not return as the block's result.
        // without explicitly specify, a () will be returned.
        // the second line doesn't end with semicolon, which means the evaluation result of the expression would be returned as the block's result.
        x + 1
    };
    println!("The value of y is : {}", y);
}
