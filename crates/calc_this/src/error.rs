use std::{fmt::Debug, num::ParseFloatError};

use crate::{ast::Operator, token::Token};

#[derive(Debug)]
pub enum CalcError<'a> {
    DivisionByZero,
    IllegelOperator(Operator),
    InvalidCharacter(char),
    InvalidToken(Token<'a>),
    MissingToken,
    MissingParenthesis,
    ParseError(ParseFloatError),
    UndefinedVariable(&'a str),
}
