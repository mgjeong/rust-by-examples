mod common;

#[test]
fn test_add() {
    common::setup();
    assert_eq!(adder::add(3, 2), 5);
}
