use std::collections::HashSet;

#[derive(Debug)]
pub enum Token<'a> {
    OpenParen,
    ClosedParen,
    LogicalOr,
    LogicalAnd,
    LogicalImp,
    LogicalNot,
    Variable(&'a str),
}

pub fn tokenize<'a>(
    input: &'a str,
) -> Result<(Vec<Token<'a>>, HashSet<&'a str>), String> {
    let mut tokens = Vec::with_capacity(input.len());
    let mut variables = HashSet::new();

    for i in 0..input.len() {
        let curr_char = input.get(i..i + 1).unwrap();

        match curr_char {
            "(" => tokens.push(Token::OpenParen),
            ")" => tokens.push(Token::ClosedParen),
            "v" => tokens.push(Token::LogicalOr),
            "&" => tokens.push(Token::LogicalAnd),
            "~" => tokens.push(Token::LogicalNot),
            "-" => match input.get(i + 1..i + 2) {
                None => {
                    return Err(format!(
                        "Unexpected end of input at index {}",
                        i + 2
                    ))
                }
                Some(c) => {
                    if c == ">" {
                        tokens.push(Token::LogicalImp)
                    } else {
                        return Err(format!("Unexpected token {} at position {}. Expected \">\"", c, i+2));
                    }
                }
            },
            "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "K"
            | "L" | "M" | "N" | "O" | "P" | "Q" | "R" | "S" | "T" | "U"
            | "V" | "W" | "X" | "Y" | "Z" => {
                tokens.push(Token::Variable(curr_char));
                variables.insert(curr_char);
            }
            " " | ">" => (),
            _ => return Err(format!("Invalid character {}", curr_char)),
        }
    }

    Ok((tokens, variables))
}
