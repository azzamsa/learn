/// Return an array of the first `n` multiples of `x`
///
/// [Training on Count by X | Codewars](https://www.codewars.com/kata/5513795bd3fafb56c200049e/rust)
///
/// Level: 8 Kyu
///
/// Tags: Arrays, Fundamentals
pub fn count_by(x: u32, n: u32) -> Vec<u32> {
    (1..=n).map(|i| i * x).collect()
}

#[cfg(test)]
mod tests {
    use super::count_by;

    #[test]
    fn test_count_by() {
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(count_by(1, 10), expected);

        let expected = vec![2, 4, 6, 8, 10];
        assert_eq!(count_by(2, 5), expected);

        let expected = vec![3, 6, 9, 12, 15, 18, 21];
        assert_eq!(count_by(3, 7), expected);

        let expected = vec![50, 100, 150, 200, 250];
        assert_eq!(count_by(50, 5), expected);

        let expected = vec![100, 200, 300, 400, 500, 600];
        assert_eq!(count_by(100, 6), expected);
    }
}
