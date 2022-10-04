/// Find Maximum and Minimum Values of a List
///
/// [Training on Find Maximum and Minimum Values of a List | Codewars](https://www.codewars.com/kata/577a98a6ae28071780000989/train/rust)
///
/// Level: kyu 8
///
/// Tags: Fundamentals

/// Find minimum value of an array.
pub fn minimum(arr: &[i32]) -> i32 {
    let mut arr_: Vec<&i32> = arr.iter().collect();
    arr_.sort();
    *arr_[0]
}

/// Find maximum value of an array.
pub fn maximum(arr: &[i32]) -> i32 {
    let mut arr_: Vec<&i32> = arr.iter().collect();
    arr_.sort();
    *arr_[arr_.len() - 1]
}

/// Other's solution.
///
/// I did't look into what functions iter has closely.
/// [Iter in std::slice - Rust](https://doc.rust-lang.org/std/slice/struct.Iter.html)
pub fn minimum_other(arr: &[i32]) -> i32 {
    *arr.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::{maximum, minimum};

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(arr: &[i32], expected_min: i32, expected_max: i32) {
        assert_eq!(
            minimum(arr),
            expected_min,
            "{ERR_MSG} with function minimum and arr = {arr:?}"
        );
        assert_eq!(
            maximum(arr),
            expected_max,
            "{ERR_MSG} with function maximum and arr = {arr:?}"
        );
    }

    #[test]
    fn fixed_tests() {
        dotest(&[-52, 56, 30, 29, -54, 0, -110], -110, 56);
        dotest(&[42, 54, 65, 87, 0], 0, 87);
        dotest(&[1, 2, 3, 4, 5, 10], 1, 10);
        dotest(
            &[
                -1, -2, -3, -4, -5, -10, 534, 43, 2, 1, 3, 4, 5, 5, 443, 443, 555, 555,
            ],
            -10,
            555,
        );
        dotest(&[9], 9, 9);
        dotest(&[4, 6, 2, 1, 9, 63, -134, 566], -134, 566);
    }
}
