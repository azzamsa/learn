/// What is between?
///
/// [Training on What is between? | Codewars](https://www.codewars.com/kata/55ecd718f46fba02e5000029/train/rust)
///
/// Level: kyu 8
///
/// Tags: Fundamentals, Algorithm

// My solution.
pub fn between(a: i16, b: i16) -> Vec<i16> {
    (a..=b).collect()
}

#[cfg(test)]
mod tests {
    use super::between;

    fn dotest(a: i16, b: i16, expected: &[i16]) {
        let actual = between(a, b);
        assert!(
            actual == expected,
            "Test failed with a = {a}, b = {b}\nExpected {expected:?}\nBut got {actual:?}"
        )
    }

    #[test]
    fn test_basic() {
        dotest(1, 4, &[1, 2, 3, 4]);
        dotest(-2, 2, &[-2, -1, 0, 1, 2]);
    }
}
