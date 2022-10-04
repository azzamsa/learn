/// Hello World
///
/// [Function 1 - hello world | Codewars](https://www.codewars.com/kata/523b4ff7adca849afe000035/rust)
///
/// Level: kyu 8
///
/// Tags: Fundamentals

/// My solution.
pub const fn greet() -> &'static str {
    "hello world!"
}

#[cfg(test)]
mod tests {
    use super::greet;

    #[test]
    fn test_greets_the_world() {
        assert_eq!(greet(), "hello world!", "should return the correct message");
    }
}
