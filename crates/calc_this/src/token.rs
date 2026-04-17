use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LeftParen,
    RightParen,
    Number(f64),
    Ident(&'a str),
}
