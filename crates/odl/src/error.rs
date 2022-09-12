use crate::token::*;
use core::fmt::Display;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct IndentationError {
    pub span: Span,
    pub expected: i64,
    pub actual: i64,
}

impl Display for IndentationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        return write!(f, "{}", self);
    }
}

impl Error for IndentationError {
    fn description(&self) -> &str {
        return "Line started with a unexpected ammount of whitespaces";
    }
}

#[derive(Debug, Clone)]
pub struct EndOfTokenStreamError {}

impl Display for EndOfTokenStreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        return write!(f, "{}", self);
    }
}

impl Error for EndOfTokenStreamError {
    fn description(&self) -> &str {
        return "There were no more tokens";
    }
}

#[derive(Debug, Clone)]
pub struct UnexpectedTokenError {
    pub token: Token,
}

impl Display for UnexpectedTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        return write!(f, "{}", self);
    }
}

impl Error for UnexpectedTokenError {
    fn description(&self) -> &str {
        return "unexpected token";
    }
}

#[derive(Debug, Clone)]
pub enum ParserError {
    EndOfTokenStream(EndOfTokenStreamError),
    Indentation(IndentationError),
    UnexpectedToken(UnexpectedTokenError),
}

impl ParserError {
    pub fn new_unexpected_token(token: Token) -> ParserError {
        return ParserError::UnexpectedToken(UnexpectedTokenError { token });
    }

    pub fn new_end_of_token_stream() -> ParserError {
        return ParserError::EndOfTokenStream(EndOfTokenStreamError {});
    }

    pub fn new_indentation_miss_match(span: Span, expected: i64, actual: i64) -> ParserError {
        return ParserError::Indentation(IndentationError {
            span,
            expected,
            actual,
        });
    }

    pub fn is_indentation_error(&self) -> bool {
        return matches!(self, ParserError::Indentation(_));
    }

    pub fn is_unexpected_token_error(&self) -> bool {
        return matches!(self, ParserError::UnexpectedToken(_));
    }

    pub fn is_end_of_token_stream(&self) -> bool {
        return matches!(self, ParserError::EndOfTokenStream(_));
    }

    pub fn get_indentation_error(&self) -> Option<&IndentationError> {
        return match self {
            ParserError::Indentation(error) => Some(error),
            _ => None,
        };
    }

    pub fn get_unexpected_token(&self) -> Option<&UnexpectedTokenError> {
        return match self {
            ParserError::UnexpectedToken(error) => Some(error),
            _ => None,
        };
    }

    pub fn get_end_of_token_stream(&self) -> Option<&EndOfTokenStreamError> {
        return match self {
            ParserError::EndOfTokenStream(error) => Some(error),
            _ => None,
        };
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        return match self {
            ParserError::EndOfTokenStream(content) => content.fmt(f),
            ParserError::Indentation(content) => content.fmt(f),
            ParserError::UnexpectedToken(content) => content.fmt(f),
        };
    }
}

impl Error for ParserError {
    fn description(&self) -> &str {
        return match self {
            ParserError::EndOfTokenStream(content) => content.description(),
            ParserError::Indentation(content) => content.description(),
            ParserError::UnexpectedToken(content) => content.description(),
        };
    }
}
