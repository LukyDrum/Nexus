use crate::parsing::{
    Metadata,
    token::{Token, TokenWithMeta, Value},
};

#[derive(Clone, Debug)]
pub enum ScannerError {
    UnexpectedChar(CharWithMeta),
    UnterminatedString(Metadata),
}

#[derive(Clone, Debug)]
pub struct CharWithMeta {
    char: char,
    meta: Metadata,
}

pub(super) fn scan<'a>(input: &'a str) -> Result<Vec<TokenWithMeta<'a>>, ScannerError> {
    let lines = input.split_inclusive('\n');
    let mut chars = lines
        .into_iter()
        .enumerate()
        .flat_map(|(line_num, line)| {
            line.chars()
                .enumerate()
                .map(move |(col, char)| CharWithMeta {
                    char,
                    meta: Metadata {
                        line: line_num,
                        col,
                    },
                })
        })
        .enumerate()
        .peekable();

    let mut tokens = Vec::new();

    while let Some((index, char)) = chars.next() {
        let CharWithMeta { char, meta } = char;

        let token = match char {
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '[' => Token::LeftBracket,
            ']' => Token::RightBracket,
            '=' => Token::Equal,
            ',' => Token::Comma,
            quote @ ('"' | '\'') => {
                let Some(string) = scan_string(input, index, quote) else {
                    return Err(ScannerError::UnterminatedString(meta));
                };

                // Skip over the string
                for _ in 0..string.len() + 1 {
                    let _ = chars.next();
                }

                Token::Value(Value::String(string))
            }
            c if c.is_whitespace() => continue,
            c if is_ident_char(c) => {
                let start = index;
                let mut end = start;
                while chars
                    .next_if(|(_, CharWithMeta { char, .. })| is_ident_char(*char))
                    .is_some()
                {
                    end += 1;
                }

                Token::Ident(&input[start..=end])
            }
            _ => return Err(ScannerError::UnexpectedChar(CharWithMeta { char, meta })),
        };

        tokens.push(TokenWithMeta { token, meta });
    }

    Ok(tokens)
}

fn scan_string(input: &str, start_index: usize, start_quote: char) -> Option<&str> {
    let start = start_index + 1;
    if start >= input.len() {
        return None;
    }

    let mut end = start;
    for char in input[start..].chars() {
        match char {
            quote if quote == start_quote => return Some(&input[start..end]),
            '\n' => return None,
            _ => end += 1,
        }
    }

    None
}

fn is_ident_char(char: char) -> bool {
    char.is_alphanumeric() || char == '_'
}
