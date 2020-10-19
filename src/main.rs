mod evaluator;
mod parser;

use evaluator::{evaluate, Expr, Operator};
use parser::tokenize;
use std::collections::HashMap;

fn main() -> Result<(), String> {
    let str_expr = "(A -> B) & (A v B)";
    println!("Expr: {}", str_expr);
    let (tokens, variables) = tokenize(str_expr)?;
    println!("{:#?}", tokens);
    println!("{:#?}", variables);
    Ok(())
}
