use pest::Parser;
use pest_derive::Parser;

use crate::Expr;
use crate::Operator;

#[derive(Parser)]
#[grammar = "calc.pest"]
struct Calculator;

pub fn parse(raw_expr: &str) -> Expr {
    let mut lhs: Option<i32> = None;
    let mut rhs: Option<i32> = None;
    let mut operator: Option<Operator> = None;

    let pair = Calculator::parse(Rule::spec, raw_expr)
        .unwrap()
        .next()
        .unwrap();
    dbg!("{:#?}", &pair);

    for piece in pair.into_inner() {
        // dbg!("{:#?}", &piece);
        match piece.as_rule() {
            Rule::lhs => lhs = Some(piece.as_str().parse().unwrap()),
            Rule::rhs => rhs = Some(piece.as_str().parse().unwrap()),
            Rule::add => operator = Some(Operator::Add),
            Rule::subtract => operator = Some(Operator::Subtract),
            _ => unreachable!(),
        }
    }

    Expr { lhs, rhs, operator }
}
