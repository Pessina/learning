use tests;

pub mod common;

#[test]
fn it_add() {
    assert_eq!(tests::add(1, 2), 3)
}
