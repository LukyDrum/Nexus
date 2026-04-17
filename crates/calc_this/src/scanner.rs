use std::{fmt::Debug, str::FromStr};

use crate::{error::CalcError, token::Token};

pub(crate) fn scan<'a, T>(
    input: &'a str,
) -> Result<Vec<Token<'a, T>>, CalcError<<T as FromStr>::Err>>
where
    T: FromStr + PartialEq,
    T::Err: Debug,
{
    let mut input_iter = input.chars().enumerate().peekable();
    let mut tokens = Vec::new();

    while let Some((index, char)) = input_iter.next() {
        let token = match char {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::Slash,
            '^' => Token::Caret,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            c if is_valid_number_char(c) => {
                let start = index;
                while input_iter
                    .peek()
                    .is_some_and(|(_, c)| is_valid_number_char(*c))
                {
                    let _ = input_iter.next();
                }

                let (end, _) = input_iter
                    .peek()
                    .map(|(i, a)| (*i, *a))
                    .unwrap_or((input.len(), '_'));

                Token::Number(
                    input[start..end]
                        .parse::<T>()
                        .map_err(CalcError::ParseError)?,
                )
            }
            c if is_valid_ident_char(c) => {
                let start = index;
                while input_iter
                    .peek()
                    .is_some_and(|(_, c)| is_valid_ident_char(*c))
                {
                    let _ = input_iter.next();
                }
                let (end, _) = input_iter
                    .peek()
                    .map(|(i, a)| (*i, *a))
                    .unwrap_or((input.len(), '_'));

                Token::Ident(&input[start..end])
            }
            ' ' => continue,
            c => return Err(CalcError::InvalidCharacter(c)),
        };

        tokens.push(token);
    }

    Ok(tokens)
}

fn is_valid_number_char(c: char) -> bool {
    c.is_ascii_digit() || c == '.'
}

fn is_valid_ident_char(c: char) -> bool {
    c.is_ascii_alphanumeric()
}

#[cfg(test)]
mod tests {
    use crate::{scanner::scan, token::Token};

    #[test]
    fn integer_math() {
        let expr = "(5 + 42) * 10 / x - 5";
        let tokens = scan::<i32>(expr).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::LeftParen,
                Token::Number(5),
                Token::Plus,
                Token::Number(42),
                Token::RightParen,
                Token::Star,
                Token::Number(10),
                Token::Slash,
                Token::Ident("x"),
                Token::Minus,
                Token::Number(5),
            ]
        );
    }

    #[test]
    fn floats() {
        let expr = "5.0 + 10";
        let tokens = scan::<f32>(expr).unwrap();

        assert_eq!(
            tokens,
            vec![Token::Number(5.0), Token::Plus, Token::Number(10.0),]
        );
    }

    #[test]
    fn function() {
        let expr = "sin(42)";
        let tokens = scan::<i32>(expr).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Ident("sin"),
                Token::LeftParen,
                Token::Number(42),
                Token::RightParen,
            ]
        );
    }
}
