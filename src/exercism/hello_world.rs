// &'static is a "lifetime specifier", something you'll learn more about later
pub fn hello() -> &'static str {
    "Hello, World!"
}

#[test]
fn hello_world() {
    assert_eq!("Hello, World!", hello());
}
