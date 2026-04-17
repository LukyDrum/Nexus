use crate::{error::CalcError, interpreter::evaluate, parser::parse, scanner::scan};

mod ast;
mod error;
mod interpreter;
mod parser;
mod scanner;
mod token;

pub fn calc_this<'a>(input: &'a str, variables: &[(&str, f64)]) -> Result<f64, CalcError<'a>> {
    let tokens = scan(input)?;
    let ast = parse(tokens)?;
    evaluate(ast, variables)
}

#[cfg(test)]
mod tests {
    use crate::{calc_this, error::CalcError};

    #[test]
    fn addition() {
        let res = calc_this("5 + 5", &[]).unwrap();
        assert_eq!(res, 10.0);
    }

    #[test]
    fn subtraction() {
        let res = calc_this("5 - 5", &[]).unwrap();
        assert_eq!(res, 0.0);
    }

    #[test]
    fn multiplication() {
        let res = calc_this("5 * 5", &[]).unwrap();
        assert_eq!(res, 25.0);
    }

    #[test]
    fn division() {
        let res = calc_this("5 / 5", &[]).unwrap();
        assert_eq!(res, 1.0);
    }

    #[test]
    fn division_by_zero() {
        let res = calc_this("5 / 0", &[]);
        assert!(matches!(res, Err(CalcError::DivisionByZero)))
    }

    #[test]
    fn power() {
        let res = calc_this("5 ^ 3", &[]).unwrap();
        assert_eq!(res, 125.0);
    }

    #[test]
    fn function_call() {
        let res = calc_this("sqrt(25)", &[]).unwrap();
        assert_eq!(res, 5.0);
    }

    #[test]
    fn complex() {
        let res = calc_this("sqrt(5 ^ (1.5 - -1.5) / sqrt(25))", &[]).unwrap();
        assert_eq!(res, 5.0);
    }

    #[test]
    fn variable() {
        let res = calc_this("5 * x", &[("x", 5.0)]).unwrap();
        assert_eq!(res, 25.0);
    }
}
