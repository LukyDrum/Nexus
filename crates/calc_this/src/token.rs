use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Token<'a, T>
where
    T: PartialEq,
{
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LeftParen,
    RightParen,
    Number(T),
    Ident(&'a str),
}
