#![allow(unused_assignments)]
/// Function
///
/// Hint: if you define a function, the data it accepts are called parameters.
/// If you call that function and pass data to it, then it's called arguments.
///

/// `add` is the name of the function.
/// `x` and `y` is the **parameters**.
/// The `i32` after `->` denotes that the function return a value
/// with `i32` type. Without it, the default return value is a tuple `()`.
pub fn add(x: i32, y: i32) -> i32 {
    // Rust's function doesn't need the `return` keyword most of the time.
    // Omitting `;` in the last expression equals `return x + y`.
    x + y
}
