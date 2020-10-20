use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq)]
pub enum Token<'a> {
    OpenParen,
    ClosedParen,
    LogicalOr,
    LogicalAnd,
    LogicalImp,
    LogicalNot,
    Variable(&'a str),
    Value(bool),
}

impl<'a> From<&'a str> for Token<'a> {
    fn from(input: &'a str) -> Self {
        match input {
            "(" => Self::OpenParen,
            ")" => Self::ClosedParen,
            "v" => Self::LogicalOr,
            "&" => Self::LogicalAnd,
            ">" => Self::LogicalImp,
            "~" => Self::LogicalNot,
            "true" => Self::Value(true),
            "false" => Self::Value(false),
            e => Self::Variable(e),
        }
    }
}

pub fn shunting_yard<'a>(
    input: &'a str,
) -> Result<(Vec<Token<'a>>, HashSet<&'a str>), String> {
    let mut output = Vec::new();
    let mut operator_stack = Vec::new();
    let mut variables = HashSet::new();

    for i in 0..input.len() {
        let curr_char = input.get(i..i + 1).unwrap();
        match curr_char {
            "&" | "v" | ">" | "~" => {
                while let Some(t) = operator_stack.last() {
                    if curr_char != "~" && *t == Token::OpenParen {
                        break;
                    } else if curr_char == "~" && *t == Token::LogicalNot {
                        break;
                    }
                    output.push(operator_stack.pop().unwrap());
                }

                operator_stack.push(curr_char.into())
            }
            "(" => operator_stack.push(Token::OpenParen),
            ")" => {
                while let Some(t) = operator_stack.last() {
                    if *t == Token::OpenParen {
                        break;
                    }
                    output.push(operator_stack.pop().unwrap())
                }

                if let Some(t) = operator_stack.last() {
                    if *t == Token::OpenParen {
                        operator_stack.pop();
                    }
                }
            }
            "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "K"
            | "L" | "M" | "N" | "O" | "P" | "Q" | "R" | "S" | "T" | "U"
            | "V" | "W" | "X" | "Y" | "Z" => {
                output.push(Token::Variable(curr_char));
                variables.insert(curr_char);
            }
            " " => (),
            "-" => {
                if let Some(next_char) = input.get(i + 1..i + 2) {
                    if next_char != ">" {
                        return Err(format!(
                            "Expected '>' following '-' char, found '{}'",
                            next_char
                        ));
                    }
                }
            }
            _ => return Err(format!("Invalid character '{}'", curr_char)),
        }
    }

    while let Some(t) = operator_stack.pop() {
        output.push(t);
    }

    Ok((output, variables))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_var() {
        let expr = "A";
        let result = shunting_yard(expr);
        assert!(result.is_ok());
        let (tokens, vars) = result.unwrap();
        assert_eq!(tokens, vec![Token::Variable("A")]);
        assert!(vars.contains("A"));
        assert_eq!(vars.len(), 1);
    }

    #[test]
    fn test_simple_expr1() {
        let expr = "A -> B";
        let result = shunting_yard(expr);
        assert!(result.is_ok());
        let (tokens, vars) = result.unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Variable("A"),
                Token::Variable("B"),
                Token::LogicalImp
            ]
        );
        assert!(vars.contains("A"));
        assert!(vars.contains("B"));
        assert_eq!(vars.len(), 2);
    }

    #[test]
    fn test_simple_expr2() {
        let expr = "A v B";
        let result = shunting_yard(expr);
        assert!(result.is_ok());
        let (tokens, vars) = result.unwrap();
        assert_eq!(
            tokens,
            vec![Token::Variable("A"), Token::Variable("B"), Token::LogicalOr]
        );
        assert!(vars.contains("A"));
        assert!(vars.contains("B"));
        assert_eq!(vars.len(), 2);
    }

    #[test]
    fn test_simple_expr3() {
        let expr = "C & D";
        let result = shunting_yard(expr);
        assert!(result.is_ok());
        let (tokens, vars) = result.unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Variable("C"),
                Token::Variable("D"),
                Token::LogicalAnd
            ]
        );
        assert!(vars.contains("C"));
        assert!(vars.contains("D"));
        assert_eq!(vars.len(), 2);
    }

    #[test]
    fn test_invalid_imp() {
        let expr = "C - > D";
        let result = shunting_yard(expr);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_char() {
        let expr = "C -> ?";
        let result = shunting_yard(expr);
        assert!(result.is_err());
    }
}
