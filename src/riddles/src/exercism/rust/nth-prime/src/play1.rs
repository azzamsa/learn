pub fn nth(n: u32) -> u32 {
    (2..)
        .inspect(|&x| println!("x: {x}"))
        .filter(|x| is_prime(*x))
        .inspect(|&x| println!("filtered: {x}"))
        .nth(n as usize)
        .unwrap()
}

pub fn is_prime(n: u32) -> bool {
    let sqrt_n = (n as f32).sqrt() as u32;
    (2..=sqrt_n).all(|x| n % x != 0)
}
