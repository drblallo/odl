use crate::token::Span;
use core::fmt::Display;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct IndentationMissMatchError {
    pub span: Span,
    pub expected: i64,
    pub actual: i64,
}

impl Display for IndentationMissMatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        return write!(f, "{}", self);
    }
}

impl Error for IndentationMissMatchError {
    fn description(&self) -> &str {
        return "Line started with a unexpected ammount of whitespaces";
    }
}
