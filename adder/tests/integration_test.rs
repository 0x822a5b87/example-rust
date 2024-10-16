mod common;

use adder::add_two;

#[test]
fn it_adds_two() {
    common::setup();

    let ret = add_two(1);
    assert_eq!(ret, 3)
}