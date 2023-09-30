pub fn reverse(input: &str) -> String {
    // https://doc.rust-lang.org/std/primitive.str.html#method.chars
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.rev
    input.chars().rev().collect()
}
