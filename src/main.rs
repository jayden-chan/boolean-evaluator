mod evaluator;
mod parser;

use evaluator::evaluate_postfix;
use parser::shunting_yard;
use std::collections::HashMap;

fn bit_at(num: usize, i: usize) -> bool {
    if i < std::mem::size_of::<usize>() {
        num & (1 << i) != 0
    } else {
        false
    }
}

fn main() -> Result<(), String> {
    let str_expr = "A -> ~B";
    let (tokens, variables) = shunting_yard(str_expr)?;
    println!("Expr: {}", str_expr);

    let mut vars = HashMap::new();
    vars.insert("A", true);
    vars.insert("B", false);
    println!("Vars: {:#?}", vars);
    let result = evaluate_postfix(tokens, &vars)?;
    println!("{}", result);
    Ok(())
}
