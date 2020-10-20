use crate::parser::Token;
use std::collections::HashMap;

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

pub fn evaluate<'a>(
    expr: &'a Expr,
    vars: &HashMap<&str, bool>,
) -> Result<bool, String> {
    match expr {
        Expr::Not(e) => Ok(!evaluate(e, vars)?),
        Expr::Variable(v) => match vars.get(v) {
            Some(b) => Ok(*b),
            None => Err(format!("Variable {} is undefined", v)),
        },
        Expr::SubExpr { l, o, r } => {
            let left_result = evaluate(l, &vars)?;
            let right_result = evaluate(r, &vars)?;
            Ok(match o {
                Operator::Or => left_result || right_result,
                Operator::And => left_result && right_result,
                Operator::Imp => !(left_result && !right_result),
            })
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_var() {
        let expr1 = Expr::Variable("A");
        let expr2 = Expr::Variable("B");

        let mut vars = HashMap::new();
        vars.insert("A", true);
        vars.insert("B", false);

        assert_eq!(evaluate(&expr1, &vars), Ok(true));
        assert_eq!(evaluate(&expr2, &vars), Ok(false));
    }

    #[test]
    fn test_not_var() {
        let expr1 = Expr::Not(&Expr::Variable("A"));

        let mut vars = HashMap::new();
        vars.insert("A", true);

        assert_eq!(evaluate(&expr1, &vars), Ok(false));
    }

    #[test]
    fn test_missing_var() {
        let expr1 = Expr::Not(&Expr::Variable("A"));

        let mut vars = HashMap::new();
        vars.insert("B", true);

        assert!(evaluate(&expr1, &vars).is_err());
    }

    #[test]
    fn test_simple_subexpr() {
        let expr = Expr::SubExpr {
            l: &Expr::Not(&Expr::Variable("A")),
            o: Operator::And,
            r: &Expr::Variable("B"),
        };

        let mut vars = HashMap::new();
        vars.insert("A", false);
        vars.insert("B", true);
        assert_eq!(evaluate(&expr, &vars), Ok(true));
    }

    #[test]
    fn test_nested_sub_expr() {
        // ~(A v (D -> R)) & (~~B -> C)
        let expr = Expr::SubExpr {
            l: &Expr::Not(&Expr::SubExpr {
                l: &Expr::Variable("A"),
                o: Operator::Or,
                r: &Expr::SubExpr {
                    l: &Expr::Variable("D"),
                    o: Operator::Imp,
                    r: &Expr::Variable("R"),
                },
            }),
            o: Operator::And,
            r: &Expr::SubExpr {
                l: &Expr::Not(&Expr::Not(&Expr::Variable("B"))),
                o: Operator::Imp,
                r: &Expr::Variable("C"),
            },
        };

        let mut vars = HashMap::new();
        vars.insert("A", false);
        vars.insert("B", true);
        vars.insert("C", false);
        vars.insert("D", false);
        vars.insert("R", false);
        assert_eq!(evaluate(&expr, &vars), Ok(false));
    }
}
