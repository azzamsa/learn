/// Disemvowel Trolls
///
/// [Training on Disemvowel Trolls | Codewars](https://www.codewars.com/kata/52fba66badcd10859f00097e/rust)
///
/// Level: 7 kyu
///
/// Tags: Fundamentals, Strings, Regex

/// My solution
///
pub fn disemvowel(s: &str) -> String {
    let vowels = ['a', 'A', 'e', 'E', 'i', 'I', 'o', 'O', 'u', 'U'];
    s.chars().filter(|x| !vowels.contains(x)).collect()
}

/// Other's solution
///
/// Put the vowel list directly inside the [`std::iter::Iterator::filter`].
/// Since it just a 'filter'. We can lowercase the vowel to shorter the list.
fn disemvowel_other(s: &str) -> String {
    s.chars()
        .filter(|&c| !"aeiou".contains(c.to_ascii_lowercase()))
        .collect()
}

#[test]
fn test_disemvowel() {
    assert_eq!(
        disemvowel("This website is for losers LOL!"),
        "Ths wbst s fr lsrs LL!"
    );
}
