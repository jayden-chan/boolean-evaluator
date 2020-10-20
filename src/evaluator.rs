use crate::parser::Token;
use std::collections::HashMap;

pub fn evaluate_postfix<'a>(
    expr: Vec<Token>,
    vars: &HashMap<&str, bool>,
) -> Result<bool, String> {
    let mut operand_stack = Vec::new();

    for t in expr {
        match t {
            Token::Value(_) | Token::Variable(_) => operand_stack.push(t),
            Token::LogicalOr | Token::LogicalAnd | Token::LogicalImp => {
                let (left_value, right_value) = match (operand_stack.pop(), operand_stack.pop()) {
                    (Some(Token::Variable(r)), Some(Token::Variable(l))) => {
                        match (vars.get(l), vars.get(r)) {
                            (Some(lv), Some(rv)) => (*lv, *rv),
                            (Some(_), None) => return Err(format!("Variable '{}' is undefined", l)),
                            (None, Some(_)) => return Err(format!("Variable '{}' is undefined", r)),
                            _ => return Err(format!("Multiple variables undefined"))
                        }
                    },
                    (Some(Token::Value(rv)), Some(Token::Value(lv))) => (lv, rv),
                    (Some(Token::Variable(r)), Some(Token::Value(lv))) => {
                        match vars.get(r) {
                            Some(rv) => (lv, *rv),
                            None => return Err(format!("Variable '{}' is undefined", r)),
                        }
                    },
                    (Some(Token::Value(rv)), Some(Token::Variable(l))) => {
                        match vars.get(l) {
                            Some(lv) => (*lv, rv),
                            None => return Err(format!("Variable '{}' is undefined", l)),
                        }
                    },
                    (None, _) | (_, None) => return Err(String::from("Too few arguments")),
                    _ => return Err(String::from("Tried to evaluate non-value/variable item from operand stack")),
                };

                match t {
                    Token::LogicalOr => operand_stack.push(Token::Value(left_value || right_value)),
                    Token::LogicalAnd => operand_stack.push(Token::Value(left_value && right_value)),
                    Token::LogicalImp => operand_stack.push(Token::Value(!(left_value && !right_value))),
                    _ => unreachable!(),
                }
            }
            Token::LogicalNot => {
                match operand_stack.pop() {
                    Some(Token::Variable(t)) => {
                        match vars.get(t) {
                            Some(tv) => operand_stack.push(Token::Value(!*tv)),
                            None => return Err(format!("Variable '{}' is undefined", t))
                        }
                    },
                    Some(Token::Value(t)) => operand_stack.push(Token::Value(!t)),
                    _ => return Err(String::from("Tried to evaluate non-value/variable item from operand stack")),
                }
            }
            _ => return Err(format!("Invalid token '{:?}' found in expression", t)),
        }
    }

    match operand_stack.pop() {
        Some(Token::Value(v)) => Ok(v),
        Some(_) => Err(String::from("Non-value token left on stack")),
        None => Err(String::from(
            "Operand stack is empty when it should have one item",
        )),
    }
}
