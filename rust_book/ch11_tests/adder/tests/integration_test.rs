use adder;

mod common;

#[test]
fn test_adds_two() {
    assert_eq!(4, adder::add_two(2));
    common::setup();
}