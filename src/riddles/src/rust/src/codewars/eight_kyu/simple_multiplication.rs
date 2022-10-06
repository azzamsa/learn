/// Simple Multiplication
///
/// [Training on Simple multiplication | Codewars](https://www.codewars.com/kata/583710ccaa6717322c000105/rust)
///
/// Level: kyu 8
///
/// Tags: Fundamentals

/// My solution.
pub const fn simple_multiplication(number: u8) -> u8 {
    match number.rem_euclid(2) {
        0 => number * 8,
        _ => number * 9,
    }
}

#[cfg(test)]
mod tests {
    use super::simple_multiplication;

    #[test]
    fn test_basic() {
        assert_eq!(simple_multiplication(1), 9);
        assert_eq!(simple_multiplication(2), 16);
        assert_eq!(simple_multiplication(4), 32);
        assert_eq!(simple_multiplication(5), 45);
    }
}
