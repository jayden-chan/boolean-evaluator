mod evaluator;

use evaluator::evaluate;
use std::collections::HashMap;

#[derive(Clone)]
pub enum Token {
    OpenParen,
    ClosedParen,
    LogicalOr,
    LogicalAnd,
    LogicalImp,
    LogicalNot,
    Variable(String),
}

#[derive(Copy, Clone)]
pub enum Operator {
    And,
    Or,
    Imp,
}

pub enum Expr<'a> {
    SubExpr {
        l: &'a Expr<'a>,
        o: Operator,
        r: &'a Expr<'a>,
    },
    Variable(&'a str),
    Not(&'a Expr<'a>),
}

fn main() {
    let my_expr = Expr::SubExpr {
        l: &Expr::Not(&Expr::Variable("A")),
        o: Operator::And,
        r: &Expr::Variable("B"),
    };

    let mut vars = HashMap::new();
    vars.insert("A", false);
    vars.insert("B", true);

    println!("{:#?}", evaluate(&my_expr, &vars));
}
