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

fn calculate(raw_expr: &str) -> i32 {
    let expr = parser::parse(raw_expr);
    let lhs = if let Some(lhs) = expr.lhs {
        lhs
    } else {
        panic!("Unrecogizeble number")
    };

    let rhs = if let Some(rhs) = expr.rhs {
        rhs
    } else {
        panic!("Unrecogizeble number")
    };

    dbg!(&lhs);
    dbg!(&rhs);
    match expr.operator {
        None => {
            panic!("Unrecogizeble operator")
        }
        Some(op) => match op {
            Operator::Add => lhs + rhs,
            Operator::Subtract => lhs - rhs,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc() {
        assert_eq!(calculate("1+2"), 3);
        assert_eq!(calculate("3-2"), 1);
        assert_eq!(calculate("3 - 2"), 1);
    }
}
