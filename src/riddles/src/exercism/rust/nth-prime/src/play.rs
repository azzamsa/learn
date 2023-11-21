use std::time::Instant;

/// Testing divisibility with all numbers up to `n`.
pub fn is_prime_v1(n: i32) -> bool {
    if n == 1 {
        return false;
    };

    if n == 2 {
        return true;
    };

    // There is no prime number that even but 2
    if n > 2 && n % 2 == 0 {
        return false;
    }

    // Don't use inclusive range (`=n`).
    // As we only need to test 2..n-1
    for x in 2..n {
        if n % x == 0 {
            return false;
        }
    }
    true
}

/// Testing divisibility with all numbers up to `n`.
/// Functional approach.
pub fn is_prime_v1_functional(n: i32) -> bool {
    if n == 1 {
        return false;
    };

    (2..n).all(|x| n % x != 0)
}

/// Testing divisibility with all numbers up to `√n`.
pub fn is_prime_v2(n: i32) -> bool {
    if n == 1 {
        return false;
    };

    let sqrt_n = (n as f32).sqrt() as i32;
    for x in 2..=sqrt_n {
        if n % x == 0 {
            return false;
        }
    }
    true
}

/// Testing divisibility with all numbers up to `√n`.
/// Functional approach.
pub fn is_prime_v2_functional(n: i32) -> bool {
    if n == 1 {
        return false;
    };

    let sqrt_n = (n as f32).sqrt() as i32;
    (2..=sqrt_n).all(|x| n % x != 0)
}

/// Testing divisibility with prime numbers up to `√n`.
pub fn is_prime_v3(n: i32) -> bool {
    if n == 1 {
        return false;
    };

    let sqrt_n = (n as f32).sqrt() as i32;
    for x in 2..=sqrt_n {
        if is_prime_v3(x) && n % x == 0 {
            return false;
        }
    }
    true
}

/// Testing divisibility with prime numbers up to `√n`.
/// Functional approach.
pub fn is_prime_v3_functional(n: i32) -> bool {
    if n == 1 {
        return false;
    };

    let sqrt_n = (n as f32).sqrt() as i32;
    (2..=sqrt_n)
        .filter(|&x| is_prime_v3_functional(x))
        .all(|x| n % x != 0)
}

/// Testing divisibility with all *odd* numbers up to `√n`.
pub fn is_prime_v4(n: i32) -> bool {
    if n == 1 {
        return false;
    };

    // There is no prime number that even but 2
    if n > 2 && n % 2 == 0 {
        return false;
    }

    let sqrt_n = (n as f32).sqrt() as i32;
    for x in (3..=sqrt_n).step_by(2) {
        if n % x == 0 {
            return false;
        }
    }
    true
}

/// Testing divisibility with all *odd* numbers up to `√n`.
/// Functional approach.
pub fn is_prime_v4_functional(n: i32) -> bool {
    if n == 1 {
        return false;
    };

    // There is no prime number that even but 2
    if n > 2 && n % 2 == 0 {
        return false;
    }

    let sqrt_n = (n as f32).sqrt() as i32;
    // Use `step_by(2)` to skip even divisors because we only need to check with divisor numbers that are primes.
    // Evey even number is not prime but 2, so we skip them.
    // Yes, there will be an odd number that is not prime, unless we do some recrusive call to filter only odd prime number.
    // But at least this approach is *hopefully* faster.
    (3..=sqrt_n)
        .step_by(2)
        // .inspect(|x| println!("x: {x}"))
        .all(|x| n % x != 0)
}

/// Minimalis approach for CP
pub fn is_prime_minimal(n: i32) -> bool {
    let sqrt_n = (n as f32).sqrt() as i32;
    (2..=sqrt_n).all(|x| n % x != 0)
}

fn main() {
    let number = 4999;
    println!("::: Prime numbers :::");
    println!("Number: {number}");
    println!();

    measure_time("is_prime_v1", is_prime_v1, number);
    measure_time("is_prime_v1_functional", is_prime_v1_functional, number);

    measure_time("is_prime_v2", is_prime_v2, number);
    measure_time("is_prime_v2_functional", is_prime_v2_functional, number);

    measure_time("is_prime_v3", is_prime_v3, number);
    measure_time("is_prime_v3_functional", is_prime_v3_functional, number);

    measure_time("is_prime_v4", is_prime_v4, number);
    measure_time("is_prime_v4_functional", is_prime_v4_functional, number);

    measure_time("is_prime_minmal", is_prime_minimal, number);
}

fn measure_time<F>(name: &str, func: F, number: i32)
where
    F: Fn(i32) -> bool,
{
    let now = Instant::now();
    println!("{}", func(number));
    let elapsed = now.elapsed();
    println!("`{}` Elapsed: {:.2?}", name, elapsed);
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime_v1() {
        assert!(is_prime_v1(2));
        assert!(!is_prime_v1(4));
        assert!(is_prime_v1(11));
        assert!(!is_prime_v1(15));
        assert!(is_prime_v1(23));
        assert!(is_prime_v1(4999));
    }

    #[test]
    fn test_is_prime_v1_functional() {
        assert!(is_prime_v1_functional(2));
        assert!(!is_prime_v1_functional(4));
        assert!(is_prime_v1_functional(11));
        assert!(!is_prime_v1_functional(15));
        assert!(is_prime_v1_functional(23));
        assert!(is_prime_v1_functional(4999));
    }

    #[test]
    fn test_is_prime_v2() {
        assert!(is_prime_v2(2));
        assert!(!is_prime_v2(4));
        assert!(is_prime_v2(11));
        assert!(!is_prime_v2(15));
        assert!(is_prime_v2(23));
        assert!(is_prime_v2(4999));
    }

    #[test]
    fn test_is_prime_v2_functional() {
        assert!(is_prime_v2_functional(2));
        assert!(!is_prime_v2_functional(4));
        assert!(is_prime_v2_functional(11));
        assert!(!is_prime_v2_functional(15));
        assert!(is_prime_v2_functional(23));
        assert!(is_prime_v2_functional(4999));
    }

    #[test]
    fn test_is_prime_v3() {
        assert!(is_prime_v3(2));
        assert!(!is_prime_v3(4));
        assert!(is_prime_v3(11));
        assert!(!is_prime_v3(15));
        assert!(is_prime_v3(23));
        assert!(is_prime_v3(4999));
    }

    #[test]
    fn test_is_prime_v3_functional() {
        assert!(is_prime_v3_functional(2));
        assert!(!is_prime_v3_functional(4));
        assert!(is_prime_v3_functional(11));
        assert!(!is_prime_v3_functional(15));
        assert!(is_prime_v3_functional(23));
        assert!(is_prime_v3_functional(4999));
    }

    #[test]
    fn test_is_prime_v4() {
        assert!(is_prime_v4(2));
        assert!(!is_prime_v4(4));
        assert!(is_prime_v4(11));
        assert!(!is_prime_v4(15));
        assert!(is_prime_v4(23));
        assert!(is_prime_v4(4999));
    }

    #[test]
    fn test_is_prime_v4_functional() {
        assert!(is_prime_v4_functional(2));
        assert!(!is_prime_v4_functional(4));
        assert!(is_prime_v4_functional(11));
        assert!(!is_prime_v4_functional(15));
        assert!(is_prime_v4_functional(23));
        assert!(is_prime_v4_functional(4999));
    }

    #[test]
    fn test_is_prime_minimal() {
        assert!(is_prime_minimal(2));
        assert!(!is_prime_minimal(4));
        assert!(is_prime_minimal(11));
        assert!(!is_prime_minimal(15));
        assert!(is_prime_minimal(23));
        assert!(is_prime_minimal(4999));
    }
}
