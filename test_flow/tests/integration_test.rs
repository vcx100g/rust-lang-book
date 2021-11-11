// in tests directly
use test_flow;
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, test_flow::add_two(2));
}