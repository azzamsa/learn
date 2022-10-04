/// Lucian's Luscious Lasagna Exercise
///
/// [Rust on Exercism](https://exercism.org/tracks/rust/exercises/lucians-luscious-lasagna)
///
/// Keywords: Functions,

// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
// #![allow(unused)]

/// Return expected minutes in the oven
pub const fn expected_minutes_in_oven() -> i32 {
    40
}

/// "calculate remaining minutes in oven given actual minutes in oven: {}", actual_minutes_in_oven
pub const fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
    expected_minutes_in_oven() - actual_minutes_in_oven
}

/// "calculate preparation time in minutes for number of layers: {}", number_of_layers
pub const fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    number_of_layers * 2
}

/// "calculate elapsed time in minutes for number of layers {} and actual minutes in oven {}", number_of_layers, actual_minutes_in_oven
pub const fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    preparation_time_in_minutes(number_of_layers) + actual_minutes_in_oven
}
