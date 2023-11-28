pub fn is_armstrong_number(num: u32) -> bool {
    let length = num.to_string().len() as u32;

    num.to_string()
        .chars()
        .map(|x| x.to_digit(10).and_then(|digit| digit.checked_pow(length)))
        .map(|x| x.unwrap_or(0) as u64)
        .sum::<u64>()
        == num as u64
}
