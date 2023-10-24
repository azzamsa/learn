pub mod parser;

#[derive(Debug, PartialEq)]
pub struct Expr {
    pub lhs: Option<i32>,
    pub rhs: Option<i32>,
    pub operator: Option<Operator>,
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
}

fn calculate(raw_expr: &str) -> Result<i32, String> {
    let expr = parser::parse(raw_expr)?;

    let lhs = expr.lhs.ok_or("Invalid number".to_string())?;
    let rhs = expr.rhs.ok_or("Invalid number".to_string())?;

    match expr.operator {
        None => Err("Invalid operator".to_string()),
        Some(op) => match op {
            Operator::Add => Ok(lhs + rhs),
            Operator::Subtract => Ok(lhs - rhs),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc() {
        assert_eq!(calculate("1+2"), Ok(3));
        assert_eq!(calculate("3-2"), Ok(1));
        assert_eq!(calculate("3 - 2"), Ok(1));
    }

    #[test]
    fn no_whitespace() {
        assert_eq!(calculate("1+2"), Ok(3));
        assert_eq!(calculate("3-2"), Ok(1));
    }

    #[test]
    fn invalid_number() {
        assert!(calculate("1+x").is_err());
    }

    #[test]
    fn invalid_operator() {
        assert!(calculate("1&2").is_err());
    }
}
