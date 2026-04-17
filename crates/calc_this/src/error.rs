use std::fmt::Debug;

#[derive(Debug)]
pub enum CalcError<ParseError>
where
    ParseError: Debug,
{
    InvalidCharacter(char),
    ParseError(ParseError),
}
