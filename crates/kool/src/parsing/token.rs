use crate::parsing::Metadata;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Token<'a> {
    Ident(&'a str),   // eg. Text
    Value(Value<'a>), // eg. "Hello world"
    LeftParen,        // (
    RightParen,       // )
    LeftBracket,      // [
    RightBracket,     // ]
    Equal,            // =
    Comma,            // ,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Value<'a> {
    String(&'a str),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TokenWithMeta<'a> {
    pub token: Token<'a>,
    pub meta: Metadata,
}
