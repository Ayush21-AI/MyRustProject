use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    ArithmeticOverflow,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ArithmeticOverflow => write!(f, "Arithmetic overflow during evaluation"),
        }
    }
}

impl StdError for Error {}

#[derive(Clone, Debug)]
pub enum Operator {
    Add,
    Subtract,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Subtract => write!(f, "-"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Expression {
    Constant(f64),
    BinaryExpr {
        op: Operator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Constant(val) => {
                write!(f, "{:.6}", val)
            }
            Expression::BinaryExpr { op, left, right } => {
                write!(f, "({} {} {})", left, op, right)
            }
        }
    }
}

#[derive(Clone)]
pub struct ExpressionContext;

impl ExpressionContext {
    pub fn new() -> Self {
        ExpressionContext
    }

    pub fn new_constant_expression(value: f64) -> Expression {
        Expression::Constant(value)
    }

    pub fn new_binary_expression(op: Operator, left: Expression, right: Expression) -> Expression {
        Expression::BinaryExpr {
            op,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub async fn eval(&self, expr: &Expression) -> Result<f64, Error> {
        match expr {
            Expression::Constant(val) => Ok(*val),
            Expression::BinaryExpr { op, left, right } => {
                let left_val = Box::pin(self.eval(left)).await?;
                let right_val = Box::pin(self.eval(right)).await?;
                match op {
                    Operator::Add => {
                        let result = left_val + right_val;
                        if result.is_infinite() {
                            Err(Error::ArithmeticOverflow)
                        } else {
                            Ok(result)
                        }
                    }
                    Operator::Subtract => {
                        let result = left_val - right_val;
                        if result.is_infinite() {
                            Err(Error::ArithmeticOverflow)
                        } else {
                            Ok(result)
                        }
                    }
                }
            }
        }
    }
}