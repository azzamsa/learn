#![allow(unused_assignments)]
/// Variable
///
/// There are three ways to bind a value into a varabile.
/// 1) let, 2) const, and 3) static
///

// Static variable has a fixed memory location and can act as a global variable
// Type must be provided here. Rust't doens't do type inference in `const` and `statsic`.
// All the global variable name must be SCREAMING CASE.
static SEASONS: [&str; 4] = ["Spring", "Summer", "Fall", "Winter"];

fn variables() {
    // The same rule apply for constant variable.
    const NUMBER_OF_MONTHS: u32 = 12;

    // All variable are immutable by default.
    let normal_variable = 8;
    // Use `mut` keyword to make it mutable
    let mut mutable_variable = 8;
    mutable_variable = 10;

    // Using the same name shadow the previous variable.
    let my_number = 8; // my_number value is: 8
    {
        let my_number = 1; // my_number value is: 1
    }
    // my_number value is: 8 (again)
}
