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
            "&" | "v" | ">" => {
                println!("Starting operator pop");
                while let Some(t) = operator_stack.last() {
                    if *t == Token::OpenParen {
                        break;
                    }
                    output.push(operator_stack.pop().unwrap());
                }

                operator_stack.push(curr_char.into())
            }
            "~" => {
                println!("Starting not operator pop");
                while let Some(t) = operator_stack.last() {
                    if *t != Token::LogicalNot {
                        break;
                    }
                    output.push(operator_stack.pop().unwrap());
                }

                operator_stack.push(curr_char.into())
            }
            "(" => operator_stack.push(Token::OpenParen),
            ")" => {
                println!("starting close paren pop");
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
            " " | "-" => (),
            _ => return Err(format!("Invalid character '{}'", curr_char)),
        }
    }

    println!("popping all remaining items");
    while let Some(t) = operator_stack.last() {
        output.push(operator_stack.pop().unwrap());
    }

    Ok((output, variables))
}
