// Because of macros
#![allow(unreachable_patterns)]

use crate::language::{
    EvaluationError, Expression, Function, FunctionCode, FunctionParam, Operator, Statement,
    StatementBlock, Value,
};
use crate::parsing::multi_peek::MultiPeekable;
use std::collections::HashMap;

use crate::parsing::token::{Token, TokenWithMeta};

pub type Tokens<Iter> = MultiPeekable<Iter, TokenWithMeta>;

#[derive(Clone, Debug)]
pub enum ParserError {
    DuplicateArgument(String),
    ExpectedValue,
    ExpressionEvaluation(EvaluationError),
    UndeclaredVariable(String),
    UnexpectedToken(TokenWithMeta),
    UnexpectedEof { expected: String },
    VariableRedeclaration(String),
}

macro_rules! match_token {
    ($token:expr, $pat:pat => $value:ident, expected = $expected:expr) => {
        match $token {
            Some(TokenWithMeta {
                token: $pat,
                meta: _,
            }) => $value,
            Some(token) => return Err(ParserError::UnexpectedToken(token.clone())),
            None => {
                return Err(ParserError::UnexpectedEof {
                    expected: $expected.to_owned(),
                })
            }
        }
    };
    ($token:expr, $expected:expr) => {
        match $token {
            Some(TokenWithMeta { token, meta: _ }) if token == $expected => {}
            Some(token) => return Err(ParserError::UnexpectedToken(token.clone())),
            None => {
                return Err(ParserError::UnexpectedEof {
                    expected: format!("{:?}", $expected),
                })
            }
        }
    };
}

macro_rules! if_token {
    ($token:expr, $expected:expr, $code:block) => {
        if_token!($token, $expected, $code else {})
    };
    ($token:expr, $expected:expr, $true:block else $false:block) => {
        match $token {
            Some(TokenWithMeta { token, meta: _ }) if token == $expected => $true,
            Some(_other) => $false,
            None => {
                return Err(ParserError::UnexpectedEof {
                    expected: format!("{:?}", $expected),
                })
            }
        }
    };
}

const NULL_KEYWORD: &str = "null";
const VAR_KEY_WORD: &str = "var";
const DEF_KEYWORD: &str = "def";

pub fn parse(tokens: impl Iterator<Item = TokenWithMeta>) -> Result<StatementBlock, ParserError> {
    let mut tokens = MultiPeekable::new(tokens);

    let mut root_block = Vec::new();

    while tokens.peek().is_some() {
        let statement = statement(&mut tokens)?;
        root_block.push(statement);
    }

    Ok(StatementBlock {
        statements: root_block,
    })
}

fn statement(
    tokens: &mut Tokens<impl Iterator<Item = TokenWithMeta>>,
) -> Result<Statement, ParserError> {
    let peek = match_token!(tokens.peek(), anything => anything, expected = "a statement");

    match peek {
        Token::Ident(ident) if ident == VAR_KEY_WORD => var_declaration(tokens),
        Token::Ident(ident) if ident == DEF_KEYWORD => function_definition(tokens),
        Token::Ident(_) => {
            let peek = tokens.multi_peek(2);
            match peek.get(1) {
                Some(TokenWithMeta {
                    token: Token::Equal,
                    ..
                }) => {
                    let name = match_token!(tokens.next(), Token::Ident(name) => name, expected = "a variable name");
                    var_assignment(tokens, name)
                }
                Some(TokenWithMeta {
                    token: Token::LeftParen,
                    ..
                }) => {
                    let name = match_token!(tokens.next(), Token::Ident(name) => name, expected = "a function name");
                    function_call(tokens, name)
                        .map(|expression| Statement::Expression { expression })
                }
                _ => expression(tokens).map(|expression| Statement::Expression { expression }),
            }
        }
        Token::LeftBrace => block(tokens).map(|block| Statement::Block { block }),
        _ => expression(tokens).map(|expression| Statement::Expression { expression }),
    }
}

fn block(
    tokens: &mut Tokens<impl Iterator<Item = TokenWithMeta>>,
) -> Result<StatementBlock, ParserError> {
    match_token!(tokens.next(), Token::LeftBrace);

    let mut statements = Vec::new();
    loop {
        if_token!(tokens.peek(), &Token::RightBrace, {
            let _ = tokens.next();
            break;
        });

        let statement = statement(tokens)?;
        statements.push(statement);
    }

    Ok(StatementBlock { statements })
}

fn var_declaration(
    tokens: &mut Tokens<impl Iterator<Item = TokenWithMeta>>,
) -> Result<Statement, ParserError> {
    match_token!(tokens.next(), Token::Ident(keyword) => keyword, expected = "variable declaration keyword `var`");

    let name =
        match_token!(tokens.next(), Token::Ident(ident) => ident, expected = "a variable name");
    match_token!(tokens.next(), Token::Equal);

    let expression = expression(tokens)?;

    Ok(Statement::VariableDeclaration {
        variable: name,
        right_side: expression,
    })
}

/// We expect the name of the variable to be handed to us.
fn var_assignment(
    tokens: &mut Tokens<impl Iterator<Item = TokenWithMeta>>,
    name: String,
) -> Result<Statement, ParserError> {
    match_token!(tokens.next(), Token::Equal);

    let expression = expression(tokens)?;

    Ok(Statement::VariableAssignment {
        variable: name,
        right_side: expression,
    })
}

fn function_definition(
    tokens: &mut Tokens<impl Iterator<Item = TokenWithMeta>>,
) -> Result<Statement, ParserError> {
    match_token!(tokens.next(), Token::Ident(keyword) => keyword, expected = "function definition keyword `def`");

    let name =
        match_token!(tokens.next(), Token::Ident(ident) => ident, expected = "a function name");
    match_token!(tokens.next(), Token::LeftParen);

    let mut params = Vec::new();
    loop {
        if_token!(tokens.peek(), &Token::RightParen, {
            let _ = tokens.next();
            break;
        });

        let param_name = match_token!(tokens.next(), Token::Ident(ident) => ident, expected = "a parameter name");
        let TokenWithMeta { token: peek, meta } =
            tokens.peek().ok_or(ParserError::UnexpectedEof {
                expected: "rest of function parameters".to_owned(),
            })?;

        let default = match peek {
            Token::Equal => {
                // Consume equal token
                let _ = tokens.next();
                let Expression::Value(value) = expression(tokens)? else {
                    return Err(ParserError::ExpectedValue);
                };

                Some(value)
            }
            Token::Comma => {
                let _ = tokens.next();
                None
            }
            Token::RightParen => None,
            token => {
                return Err(ParserError::UnexpectedToken(TokenWithMeta {
                    token: token.clone(),
                    meta: *meta,
                }));
            }
        };

        params.push(FunctionParam {
            name: param_name.to_owned(),
            default,
        });
    }

    let code = block(tokens)?;

    let function = Function {
        params,
        code: FunctionCode::Block(code),
    };

    Ok(Statement::FunctionDefinition {
        name: name.to_owned(),
        function,
    })
}

/// We expect the name of the function to be handed to us.
fn function_call(
    tokens: &mut Tokens<impl Iterator<Item = TokenWithMeta>>,
    name: String,
) -> Result<Expression, ParserError> {
    match_token!(tokens.next(), Token::LeftParen);

    let mut args = HashMap::new();
    let mut tail = Vec::new();

    loop {
        if_token!(tokens.peek(), &Token::RightParen, {
            let _ = tokens.next();
            break;
        });

        // 2 cases:
        //      - <param>: <value>
        //      - <value>
        let peek = tokens.multi_peek(2);
        match peek.get(1) {
            Some(TokenWithMeta {
                token: Token::Colon,
                ..
            }) => {
                let param = match_token!(tokens.next(), Token::Ident(param) => param, expected = "param name");
                match_token!(tokens.next(), Token::Colon);
                let expression = expression(tokens)?;

                if args.contains_key(&param) {
                    return Err(ParserError::DuplicateArgument(param));
                }

                args.insert(param, expression);
            }
            _ => {
                let expression = expression(tokens)?;
                tail.push(expression);
            }
        }

        // An end of the arguments or a comma must follow
        let TokenWithMeta { token: peek, meta } =
            tokens.peek().ok_or(ParserError::UnexpectedEof {
                expected: "comma or paren".to_owned(),
            })?;
        match peek {
            Token::Comma => {
                let _ = tokens.next();
            }
            Token::RightParen => {}
            token => {
                return Err(ParserError::UnexpectedToken(TokenWithMeta {
                    token: token.clone(),
                    meta: *meta,
                }));
            }
        }
    }

    Ok(Expression::FunctionCall { name, args, tail })
}

fn expression(
    tokens: &mut Tokens<impl Iterator<Item = TokenWithMeta>>,
) -> Result<Expression, ParserError> {
    let mut left = term(tokens)?;

    while tokens
        .peek()
        .is_some_and(|TokenWithMeta { token, .. }| matches!(token, Token::Plus | Token::Minus))
    {
        let Some(token) = tokens.next() else {
            break;
        };
        let operator = token_to_operator(token)?;
        let right = term(tokens)?;

        left = Expression::Binary {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        };
    }

    Ok(left)
}

fn term(
    tokens: &mut Tokens<impl Iterator<Item = TokenWithMeta>>,
) -> Result<Expression, ParserError> {
    let mut left = unary(tokens)?;

    while tokens
        .peek()
        .is_some_and(|TokenWithMeta { token, .. }| matches!(token, Token::Star | Token::Slash))
    {
        let Some(token) = tokens.next() else {
            break;
        };
        let operator = token_to_operator(token)?;
        let right = unary(tokens)?;

        left = Expression::Binary {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        };
    }

    Ok(left)
}

fn unary(
    tokens: &mut Tokens<impl Iterator<Item = TokenWithMeta>>,
) -> Result<Expression, ParserError> {
    if tokens
        .peek()
        .is_some_and(|TokenWithMeta { token, .. }| matches!(token, Token::Plus | Token::Minus))
    {
        let Some(token) = tokens.next() else {
            return Err(ParserError::UnexpectedEof {
                expected: "an operator".to_owned(),
            });
        };
        let operator = token_to_operator(token)?;
        let operand = unary(tokens)?;

        Ok(Expression::Unary {
            operator,
            operand: Box::new(operand),
        })
    } else {
        power(tokens)
    }
}

fn power(
    tokens: &mut Tokens<impl Iterator<Item = TokenWithMeta>>,
) -> Result<Expression, ParserError> {
    let mut left = primary(tokens)?;

    while tokens
        .peek()
        .is_some_and(|TokenWithMeta { token, .. }| matches!(token, Token::Caret))
    {
        let Some(token) = tokens.next() else {
            break;
        };
        let operator = token_to_operator(token)?;
        let right = primary(tokens)?;

        left = Expression::Binary {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        };
    }

    Ok(left)
}

fn primary(
    tokens: &mut Tokens<impl Iterator<Item = TokenWithMeta>>,
) -> Result<Expression, ParserError> {
    let mut expression = if let Some(TokenWithMeta {
        token: Token::LeftBracket,
        ..
    }) = tokens.peek()
    {
        parse_array(tokens)?
    } else {
        let Some(TokenWithMeta { token, meta }) = tokens.next() else {
            return Err(ParserError::UnexpectedEof {
                expected: "anything expression like".to_owned(),
            });
        };

        match token {
            Token::Ident(ident) if ident == NULL_KEYWORD => Expression::Value(Value::Null),
            // Either variable or function call
            Token::Ident(ident) => {
                if_token!(tokens.peek(), &Token::LeftParen, {
                    function_call(tokens, ident)?
                } else {
                    Expression::Variable(ident)
                })
            }
            Token::Number(num) => Expression::Value(Value::Number(num)),
            Token::String(string) => Expression::Value(Value::String(string)),
            Token::LeftParen => {
                let expression = expression(tokens)?;
                match_token!(tokens.next(), Token::RightParen);

                expression
            }
            _ => return Err(ParserError::UnexpectedToken(TokenWithMeta { token, meta })),
        }
    };

    while let Some(TokenWithMeta {
        token: Token::LeftBracket,
        ..
    }) = tokens.peek()
    {
        let index = parse_indexing(tokens)?;
        expression = Expression::Binary {
            operator: Operator::Index,
            left: Box::new(expression),
            right: Box::new(index),
        };
    }

    Ok(expression)
}

fn parse_array(
    tokens: &mut Tokens<impl Iterator<Item = TokenWithMeta>>,
) -> Result<Expression, ParserError> {
    match_token!(tokens.next(), Token::LeftBracket);

    let mut array = Vec::new();
    while let Some(TokenWithMeta { token, .. }) = tokens.peek() {
        match token {
            // We need this branch because of empty arrays []
            Token::RightBracket => {
                tokens.next();
                return Ok(Expression::Array(array));
            }
            _ => {
                let expression = expression(tokens)?;
                array.push(expression);
            }
        }

        let token = match_token!(tokens.peek(), t @ (Token::Comma | Token::RightBracket) => t, expected = "a comma or an end of array");
        // Discard comma
        if let Token::Comma = token {
            tokens.next();
        }
    }

    Err(ParserError::UnexpectedEof {
        expected: "an arrays next item or an end of the array".to_owned(),
    })
}

fn parse_indexing(
    tokens: &mut Tokens<impl Iterator<Item = TokenWithMeta>>,
) -> Result<Expression, ParserError> {
    match_token!(tokens.next(), Token::LeftBracket);

    let expression = expression(tokens)?;

    match_token!(tokens.next(), Token::RightBracket);

    Ok(expression)
}

fn token_to_operator(token: TokenWithMeta) -> Result<Operator, ParserError> {
    Ok(match token.token {
        Token::Plus => Operator::Add,
        Token::Minus => Operator::Sub,
        Token::Star => Operator::Mul,
        Token::Slash => Operator::Div,
        Token::Caret => Operator::Power,
        _ => return Err(ParserError::UnexpectedToken(token)),
    })
}
