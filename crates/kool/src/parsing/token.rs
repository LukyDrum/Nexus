use crate::{element::environment::Value, parsing::Metadata};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Token<'a> {
    Ident(&'a str),    // eg. Text
    Value(Value),      // eg. "Hello world"
    Variable(&'a str), // eg. $count
    LeftParen,         // (
    RightParen,        // )
    LeftBracket,       // [
    RightBracket,      // ]
    Equal,             // =
    Comma,             // ,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TokenWithMeta<'a> {
    pub token: Token<'a>,
    pub meta: Metadata,
}
