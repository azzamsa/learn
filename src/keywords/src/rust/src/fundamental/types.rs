/// Rust has simple types that are called primitive types (primitive = very basic)
///
/// The integer types are divide into: 1) signed integer, and 2) unsigned interger.
/// It is called "signed" because it has a sign. Plus (+) and minus (-) are sign.
/// Thus signed integer can be positive and negative.
/// On the other side, unsingned interger only has positive values.
///
/// The signed integers are: `i8`, `i16`, `i32`, `i64`, `i128`, and `isize`.
/// The unsigned integers are: `u8`, `u16`, `u32`, `u64`, `u128`, and `usize`.
///
/// The number after the i or the u means the number of bits for the number.
/// 8 means 8 bits. 8 bits is one byte. So i8 is one byte and i64 is eight byte.
///
fn types() {
    let small_number = 10; // by default this is i32. This is `type inference` in action!
    let small_number = 10u8;

    let small_number = 10_u8; // This is easier to read
    let big_number = 100_000_000_i32; // 100 million is easy to read with _

    let my_float = 5.; // Rust sees . and knows that it is a float. f64 by default.

    let my_bool = true;
    let my_char = 'x';

    let my_array = [1, 2, 3];
    let my_tuple = (13, false);
    let my_str = "Hello, world.";
}
