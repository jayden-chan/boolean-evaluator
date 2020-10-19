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
