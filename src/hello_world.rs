// &'static is a "lifetime specifier", something you'll learn more about later
pub fn hello() -> &'static str {
    "Hello, World!"
}

#[test]
fn hello_test() {
    assert_eq!(hello(), "Hello, World!");
}
