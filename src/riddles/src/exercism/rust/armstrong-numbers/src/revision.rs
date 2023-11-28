pub fn is_armstrong_number_v1(num: u32) -> bool {
    let mut nums: Vec<u64> = Vec::new();
    let length = num.to_string().len() as u32;

    for x in num.to_string().chars() {
        let powed = match x.to_digit(10) {
            Some(x) => x.checked_pow(length),
            None => return false,
        };

        if let Some(p) = powed {
            nums.push(p as u64)
        } else {
            return false;
        }
    }
    nums.iter().sum::<u64>() == num as u64
}

pub fn is_armstrong_number_v1a(num: u32) -> bool {
    let mut nums: Vec<u64> = Vec::new();
    let length = num.to_string().len() as u32;

    for x in num.to_string().chars() {
        let powed = if let Some(x) = x.to_digit(10) {
            x.checked_pow(length)
        } else {
            return false;
        };

        if let Some(p) = powed {
            nums.push(p as u64)
        }
    }
    nums.iter().sum::<u64>() == num as u64
}
