use the_rust_programming;

#[test]
fn it_adds_two() {
    assert_eq!(4, the_rust_programming::add_two(2));
    assert_eq!(4, the_rust_programming::libfortest::add_two(2));
}