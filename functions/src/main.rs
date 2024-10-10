#![allow(unused_variables)]

fn main() {
    let g = 0;
    let _x = 5;

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is : {}", y);

    another_function(10i32);
    let a = 10;
    another_function(a);

    expression_can_be_part_of_statements();
    calling_a_function_is_a_expression();
    calling_a_macro_is_a_expression();
    curly_bracket_is_a_expression();
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
