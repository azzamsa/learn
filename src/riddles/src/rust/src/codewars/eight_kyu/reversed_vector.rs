#![allow(clippy::cast_possible_truncation)]
/// Convert number to reversed array of digits
///
/// [Convert number to reversed array of digits | Codewars](https://www.codewars.com/kata/5583090cbe83f4fd8c000051/rust)
///
/// Level: kyu 8
///
/// Tags: Fundamentals, Arrays

/// My solution.
pub fn digitize(n: u64) -> Vec<u8> {
    let mut result = n
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .map(|x| x as u8)
        .collect::<Vec<u8>>();
    result.reverse();
    result
}

/// Other's solution.
///
/// `rev` is availiabe as functional function
/// use `as` in the same line as `to_digit`
pub fn digitize_other(n: u64) -> Vec<u8> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .rev()
        .collect::<Vec<u8>>()
}

#[cfg(test)]
mod tests {
    use super::digitize;

    #[test]
    fn test_fixed() {
        assert_eq!(digitize(348_597), vec![7, 9, 5, 8, 4, 3]);
        assert_eq!(digitize(35231), vec![1, 3, 2, 5, 3]);
        assert_eq!(digitize(0), vec![0]);
    }
}
