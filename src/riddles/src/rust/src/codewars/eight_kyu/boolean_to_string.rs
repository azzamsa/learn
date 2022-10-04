/// Convert a Boolean to a String
///
/// [Training on Convert a Boolean to a String | Codewars](https://www.codewars.com/kata/551b4501ac0447318f0009cd/train/rust)
///
/// Level: kyu 8
///
/// Tags: Fundamentals, String

// My solution.
pub fn boolean_to_string(b: bool) -> String {
    if b {
        "true".into()
    } else {
        "false".into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bolean_to_string() {
        assert_eq!(
            boolean_to_string(true),
            "true",
            "When we pass in true, we want the string \"true\" as output"
        );
        assert_eq!(
            boolean_to_string(false),
            "false",
            "When we pass in false, we want the string \"false\" as output"
        );
        assert_eq!(
            boolean_to_string(false),
            "false",
            "When we pass in false, we want the string \"false\" as output"
        );
    }
}
