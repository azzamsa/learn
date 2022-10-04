/// Convert number to reversed array of digits
///
/// [Function 1 - hello world | Codewars](https://www.codewars.com/kata/523b4ff7adca849afe000035/rust)
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

#[cfg(test)]
mod tests {
    use super::digitize;

    #[test]
    fn test_fixed() {
        assert_eq!(digitize(348597), vec![7, 9, 5, 8, 4, 3]);
        assert_eq!(digitize(35231), vec![1, 3, 2, 5, 3]);
        assert_eq!(digitize(0), vec![0]);
    }
}
