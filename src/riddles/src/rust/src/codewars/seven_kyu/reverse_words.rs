/// Reverse words
///
/// [Training on Reverse words | Codewars](https://www.codewars.com/kata/5259b20d6021e9e14c0010d4/rust)
///
/// Level: 7 kyu
///
/// Tags: Fundamentals, String

/// My solution ⚠️
///
/// It doesn't pass the test for double spaced words.
/// Turns out the output of [`str::split_whitespace`] and [`str::split`] is difference.
pub fn reverse_words(str: &str) -> String {
    str.split_whitespace()
        .map(|sub| sub.chars().rev().collect())
        .collect::<Vec<String>>()
        .join(" ")
}

/// Other's solution
///
pub fn reverse_words_other(str: &str) -> String {
    str.split(' ')
        .map(|sub| sub.chars().rev().collect())
        .collect::<Vec<String>>()
        .join(" ")
}

#[test]
fn test_reverse_words() {
    assert_eq!(
        reverse_words_other("The quick brown fox jumps over the lazy dog."),
        "ehT kciuq nworb xof spmuj revo eht yzal .god"
    );
    assert_eq!(reverse_words_other("apple"), "elppa");
    assert_eq!(reverse_words_other("a b c d"), "a b c d");
    assert_eq!(
        reverse_words_other("double  spaced  words"),
        "elbuod  decaps  sdrow"
    );
}
