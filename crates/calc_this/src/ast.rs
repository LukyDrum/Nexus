#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
}

#[derive(Debug)]
pub(crate) enum Function {
    Sqrt,
    Sin,
    Cos,
    Sign,
}

pub(crate) enum AstNode<'a> {
    Value(f64),
    Variable(&'a str),
    Function {
        function: Function,
        operand: Box<AstNode<'a>>,
    },
    Unary {
        operator: Operator,
        operand: Box<AstNode<'a>>,
    },
    Binary {
        left: Box<AstNode<'a>>,
        right: Box<AstNode<'a>>,
        operator: Operator,
    },
}
