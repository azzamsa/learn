/// Hello world
///
/// [Exercism](https://exercism.org/tracks/rust/exercises/hello-world)
///

// The &'static here means the return type has a static lifetime.
// This is a Rust feature that you don't need to worry about now.
pub const fn hello() -> &'static str {
    "Hello, World!"
}

#[test]
fn test_hello_world() {
    assert_eq!("Hello, World!", hello());
}
