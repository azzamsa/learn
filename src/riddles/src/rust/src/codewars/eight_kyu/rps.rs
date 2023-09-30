/// Rock Paper Scissors
///
/// [Rock Paper Scissors! | Codewars](https://www.codewars.com/kata/5672a98bdbdd995fad00000f/rust)
/// Level: kyu 8
///
/// Tags: Fundamentals

/// My solution.
pub fn rps(p1: &str, p2: &str) -> &'static str {
    match (p1, p2) {
        ("rock", "scissors") | ("paper", "rock") | ("scissors", "paper") => "Player 1 won!",
        ("scissors", "rock") | ("paper", "scissors") | ("rock", "paper") => "Player 2 won!",
        ("scissors", "scissors") | ("paper", "paper") | ("rock", "rock") => "Draw!",
        (&_, _) => unreachable!(),
    }
}

/// Other solution
///
/// Use or for match for shorter code
/// ues one line if for draw
fn rps_other(p1: &str, p2: &str) -> &'static str {
    if p1 == p2 {
        return "Draw!";
    }
    match (p1, p2) {
        ("scissors", "paper") | ("paper", "rock") | ("rock", "scissors") => "Player 1 won!",
        _ => "Player 2 won!",
    }
}

#[cfg(test)]
mod tests {
    use super::rps;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(p1: &str, p2: &str, expected: &str) {
        assert_eq!(
            rps(p1, p2),
            expected,
            "{ERR_MSG} with p1 = \"{p1}\", p2 = \"{p2}\""
        );
    }

    #[test]
    fn fixed_tests() {
        dotest("rock", "scissors", "Player 1 won!");
        dotest("scissors", "rock", "Player 2 won!");
        dotest("rock", "rock", "Draw!");
    }
}
