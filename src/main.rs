mod evaluator;
mod parser;

use evaluator::{evaluate, Expr, Operator};
use parser::{shunting_yard, tokenize, Token};
use std::collections::HashMap;

fn bit_at(num: usize, i: usize) -> bool {
    if i < std::mem::size_of::<usize>() {
        num & (1 << i) != 0
    } else {
        false
    }
}

fn format_expr(expr: &Vec<Token>) -> String {
    let mut ret = String::new();
    for token in expr {
        ret.push_str(match token {
            Token::OpenParen => "(",
            Token::ClosedParen => ")",
            Token::LogicalOr => " | ",
            Token::LogicalAnd => " & ",
            Token::LogicalImp => " -> ",
            Token::LogicalNot => "!",
            Token::Variable(v) => v,
        });
    }
    return ret;
}

fn main() -> Result<(), String> {
    let str_expr = "(A -> ~B) & (A v B)";
    let (tokens, variables) = shunting_yard(str_expr)?;
    println!("Expr: {}", format_expr(&tokens));
    println!("{:#?}", tokens);
    println!("{:?}", variables);
    Ok(())
}
