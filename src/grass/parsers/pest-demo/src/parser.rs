use pest::Parser;
use pest_derive::Parser;

use crate::Expr;
use crate::Operator;

#[derive(Parser)]
#[grammar = "calc.pest"]
struct Calculator;

pub fn parse(raw_expr: &str) -> Result<Expr, String> {
    let mut lhs: Option<i32> = None;
    let mut rhs: Option<i32> = None;
    let mut operator: Option<Operator> = None;

    let pair = Calculator::parse(Rule::spec, raw_expr)
        .map_err(|e| format!("Parse error: {}", e))?
        .next()
        .ok_or("No pairs found")?;

    for piece in pair.into_inner() {
        match piece.as_rule() {
            Rule::lhs => {
                lhs = Some(
                    piece
                        .as_str()
                        .parse()
                        .map_err(|e| format!("Failed to parse lhs: {}", e))?,
                )
            }
            Rule::rhs => {
                rhs = Some(
                    piece
                        .as_str()
                        .parse()
                        .map_err(|e| format!("Failed to parse rhs: {}", e))?,
                )
            }
            Rule::add => operator = Some(Operator::Add),
            Rule::subtract => operator = Some(Operator::Subtract),
            _ => unreachable!(), // This should not happen due to the grammar.
        }
    }

    // Check if the necessary parts are found
    if lhs.is_none() || rhs.is_none() || operator.is_none() {
        return Err("Invalid expression: missing parts".to_string());
    }

    Ok(Expr { lhs, rhs, operator })
}
