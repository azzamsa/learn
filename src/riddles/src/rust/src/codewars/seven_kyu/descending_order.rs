/// Return digits in descending order
///
/// [Training on Descending Order | Codewars](https://www.codewars.com/kata/5467e4d82edf8bbf40000155/rust)
///
/// Level: 7 kyu
///
/// Tags: Fundamentals

/// My solution
///
/// Other solutions:
/// One of them is using sort and reverse in one line ` result.sort_by(|a, b| b.cmp(a));`
/// This shorten the code.
pub fn descending_order(x: u64) -> u64 {
    let mut result = x
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    // `sort_unstable` is faster for primitive types
    result.sort_unstable();
    result.reverse();
    result
        .iter()
        .map(<u32>::to_string)
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}

#[test]
fn test_descending_order() {
    assert_eq!(descending_order(0), 0);
    assert_eq!(descending_order(1), 1);
    assert_eq!(descending_order(15), 51);
    assert_eq!(descending_order(1021), 2110);
    assert_eq!(descending_order(123456789), 987654321);
    assert_eq!(descending_order(145263), 654321);
    assert_eq!(descending_order(1254859723), 9875543221);
}
