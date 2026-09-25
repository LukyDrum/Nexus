use std::num::ParseIntError;

use crate::parsing::{
    Metadata,
    token::{Token, TokenWithMeta},
};

const MULTILINE_STRING_SIGN: &str = r#"""""#;

#[derive(Clone, Debug)]
pub enum ScannerError {
    EmptyVarName(Metadata),
    NumberParse(ParseIntError),
    UnexpectedChar(CharWithMeta),
    UnterminatedString(Metadata),
}

#[derive(Clone, Debug)]
pub struct CharWithMeta {
    char: char,
    meta: Metadata,
}

pub(super) fn scan(input: &str) -> Result<Vec<TokenWithMeta>, ScannerError> {
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
                        line: line_num + 1,
                        col: col + 1,
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
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            ':' => Token::Colon,
            ',' => Token::Comma,
            '.' => Token::Dot,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '?' => Token::QuestionMark,
            '/' => {
                if chars.next_if(|(_, c)| c.char == '/').is_some() {
                    while chars.next_if(|(_, c)| c.char != '\n').is_some() {}
                    continue;
                } else {
                    Token::Slash
                }
            }
            '*' => Token::Star,
            '^' => Token::Caret,
            '=' => {
                if chars.next_if(|(_, c)| c.char == '=').is_some() {
                    Token::EqualEqual
                } else {
                    Token::Equal
                }
            }
            '!' => {
                if chars.next_if(|(_, c)| c.char == '=').is_some() {
                    Token::BangEqual
                } else {
                    Token::Bang
                }
            }
            '<' => {
                if chars.next_if(|(_, c)| c.char == '=').is_some() {
                    Token::LessEqual
                } else {
                    Token::Less
                }
            }
            '>' => {
                if chars.next_if(|(_, c)| c.char == '=').is_some() {
                    Token::GreaterEqual
                } else {
                    Token::Greater
                }
            }
            '&' => {
                if chars.next_if(|(_, c)| c.char == '&').is_some() {
                    Token::And
                } else {
                    return Err(ScannerError::UnexpectedChar(CharWithMeta { char, meta }));
                }
            }
            '|' => {
                if chars.next_if(|(_, c)| c.char == '|').is_some() {
                    Token::Or
                } else {
                    return Err(ScannerError::UnexpectedChar(CharWithMeta { char, meta }));
                }
            }
            quote @ ('"' | '\'') => {
                let (string, skip) = if &input[index..index + 3] == MULTILINE_STRING_SIGN {
                    let Some(string) = scan_multiline_string(input, index) else {
                        return Err(ScannerError::UnterminatedString(meta));
                    };

                    (
                        replace_escape_chars(&replace_consecutive_whitespace(string, Some(' '))),
                        string.len() + 6,
                    )
                } else {
                    let Some(string) = scan_string(input, index, quote) else {
                        return Err(ScannerError::UnterminatedString(meta));
                    };

                    (replace_escape_chars(string), string.len() + 1)
                };

                // Skip over the string
                for _ in 0..skip {
                    let _ = chars.next();
                }

                Token::String(string.to_owned())
            }
            c if c.is_ascii_digit() => {
                let start = index;
                let mut end = start;
                while chars
                    .next_if(|(_, CharWithMeta { char, .. })| char.is_ascii_digit())
                    .is_some()
                {
                    end += 1;
                }

                let int = input[start..=end]
                    .parse()
                    .map_err(ScannerError::NumberParse)?;

                Token::Int(int)
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

                Token::Ident(input[start..=end].to_owned())
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

    let mut is_escaped = false;
    for (end, char) in (start..).zip(input[start..].chars()) {
        match char {
            '\\' => is_escaped = true,
            quote if quote == start_quote && !is_escaped => return Some(&input[start..end]),
            '\n' => return None,
            _ => is_escaped = false,
        }
    }

    None
}

fn scan_multiline_string(input: &str, start_index: usize) -> Option<&str> {
    let start = start_index + 3;
    if start >= input.len() {
        return None;
    }

    let mut end = start;
    for index in start..input.len() - 2 {
        if &input[index..index + 3] == MULTILINE_STRING_SIGN {
            return Some(&input[start..end]);
        } else {
            end = index;
        }
    }

    None
}

fn is_ident_char(char: char) -> bool {
    char.is_alphabetic() || char == '_'
}

fn replace_consecutive_whitespace(string: &str, replacement: Option<char>) -> String {
    let mut output = String::with_capacity(string.len());
    let mut last_whitespace = true;
    for char in string.chars() {
        if char.is_whitespace() {
            if !last_whitespace {
                output.push(replacement.unwrap_or(char));
                last_whitespace = true;
            }
        } else {
            output.push(char);
            last_whitespace = false;
        }
    }

    output
}

fn replace_escape_chars(string: &str) -> String {
    string
        .replace("\\n", "\n")
        .replace("\\\"", "\"")
        .replace("\\'", "'")
        .replace("\\t", "\t")
}
