use std::iter::Peekable;

use crate::{
    ast::{AstNode, Function, Operator},
    error::CalcError,
    token::Token,
};

type ParserResult<'a> = Result<AstNode<'a>, CalcError<'a>>;

pub(crate) fn parse<'a>(tokens: Vec<Token<'a>>) -> ParserResult<'a> {
    let mut iter = tokens.into_iter().peekable();
    expression(&mut iter)
}

fn token_to_operator(token: Token<'_>) -> Result<Operator, CalcError<'_>> {
    Ok(match token {
        Token::Plus => Operator::Plus,
        Token::Minus => Operator::Minus,
        Token::Star => Operator::Star,
        Token::Slash => Operator::Slash,
        Token::Caret => Operator::Caret,
        token => return Err(CalcError::InvalidToken(token)),
    })
}

fn string_to_function(string: &str) -> Option<Function> {
    Some(match string.to_lowercase().trim() {
        "sqrt" => Function::Sqrt,
        "sin" => Function::Sin,
        "cos" => Function::Cos,
        "sign" => Function::Sign,
        _ => return None,
    })
}

fn expression<'a>(tokens: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> ParserResult<'a> {
    let mut left = term(tokens)?;

    while tokens
        .peek()
        .is_some_and(|token| matches!(token, Token::Plus | Token::Minus))
    {
        let Some(token) = tokens.next() else {
            break;
        };
        let operator = token_to_operator(token)?;
        let right = term(tokens)?;

        left = AstNode::Binary {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        };
    }

    Ok(left)
}

fn term<'a>(tokens: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> ParserResult<'a> {
    let mut left = unary(tokens)?;

    while tokens
        .peek()
        .is_some_and(|token| matches!(token, Token::Star | Token::Slash))
    {
        let Some(token) = tokens.next() else {
            break;
        };
        let operator = token_to_operator(token)?;
        let right = unary(tokens)?;

        left = AstNode::Binary {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        };
    }

    Ok(left)
}

fn unary<'a>(tokens: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> ParserResult<'a> {
    if tokens
        .peek()
        .is_some_and(|token| matches!(token, Token::Plus | Token::Minus))
    {
        let Some(token) = tokens.next() else {
            return Err(CalcError::MissingToken);
        };
        let operator = token_to_operator(token)?;
        let operand = unary(tokens)?;

        Ok(AstNode::Unary {
            operator,
            operand: Box::new(operand),
        })
    } else {
        power(tokens)
    }
}

fn power<'a>(tokens: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> ParserResult<'a> {
    let mut left = primary(tokens)?;

    while tokens
        .peek()
        .is_some_and(|token| matches!(token, Token::Caret))
    {
        let Some(token) = tokens.next() else {
            break;
        };
        let operator = token_to_operator(token)?;
        let right = primary(tokens)?;

        left = AstNode::Binary {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        };
    }

    Ok(left)
}

fn primary<'a>(tokens: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> ParserResult<'a> {
    let Some(token) = tokens.next() else {
        return Err(CalcError::MissingToken);
    };

    let node = match token {
        Token::Number(num) => AstNode::Value(num),
        Token::LeftParen => {
            let expression = expression(tokens)?;
            let _right_paren = tokens
                .next_if(|token| matches!(token, Token::RightParen))
                .ok_or(CalcError::MissingParenthesis)?;
            expression
        }
        Token::Ident(ident) => {
            if let Some(function) = string_to_function(ident) {
                let _left_paren = tokens
                    .next_if(|token| matches!(token, Token::LeftParen))
                    .ok_or(CalcError::MissingParenthesis)?;
                let operand = expression(tokens)?;
                let _right_paren = tokens
                    .next_if(|token| matches!(token, Token::RightParen))
                    .ok_or(CalcError::MissingParenthesis)?;

                AstNode::Function {
                    function,
                    operand: Box::new(operand),
                }
            } else {
                AstNode::Variable(ident)
            }
        }
        token => return Err(CalcError::InvalidToken(token)),
    };

    Ok(node)
}
