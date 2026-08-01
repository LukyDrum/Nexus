use crate::parsing::Metadata;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Token {
    Ident(String),  // eg. Text
    String(String), // eg. "Hello world"
    Number(i64),    // eg. 42
    LeftParen,      // (
    RightParen,     // )
    LeftBracket,    // [
    RightBracket,   // ]
    LeftBrace,      // {
    RightBrace,     // }
    Colon,          // :
    Equal,          // =
    Comma,          // ,
    Plus,           // +
    Minus,          // -
    Slash,          // /
    Star,           // *
    Caret,          // ^
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TokenWithMeta {
    pub token: Token,
    pub meta: Metadata,
}
