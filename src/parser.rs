use crate::ast::*;
use crate::error::ParserError;
use crate::lexer::IndentLexer;
use crate::token::*;

pub struct Parser<'a> {
    lexer: IndentLexer<'a>,
    next_token: Result<Token, ParserError>,
}

#[macro_export]
macro_rules! accept {
    ($parser: expr, $pattern:pat) => {
        match $parser.peek().is_some() && matches!($parser.peek().unwrap().kind, $pattern) {
            true => {
                $parser.next()?;
                true
            }
            false => false,
        }
    };
}

#[macro_export]
macro_rules! expect {
    ($parser: expr, $pattern:pat) => {{
        if !matches!($parser.peek().unwrap().kind, $pattern) {
            return Err(ParserError::new_unexpected_token(
                $parser.peek().unwrap().clone(),
            ));
        }
        let token = $parser.next()?;
        token
    }};
}

impl<'a> Parser<'a> {
    pub fn new(s: &'a str) -> Result<Parser<'a>, ParserError> {
        let mut lexer = IndentLexer::new(s);
        let next_token = lexer.next_token();
        return Ok(Parser { lexer, next_token });
    }

    fn peek(&self) -> Option<Token> {
        return match self.next_token.as_ref() {
            Err(value) => None,
            Ok(value) => Some(value.clone()),
        };
    }

    fn next(&mut self) -> Result<Token, ParserError> {
        let to_return = self.next_token.clone();
        self.next_token = self.lexer.next_token();
        return to_return;
    }

    fn parse_int(&mut self) -> Result<Literal, ParserError> {
        let token = expect!(self, TokenKind::Integer(_i));
        return Ok(Literal::Integer(token.get_int().unwrap()));
    }

    fn parse_identifier(&mut self) -> Result<String, ParserError> {
        let token = expect!(self, TokenKind::Ident(_i));
        return Ok(token.get_identifier().unwrap());
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::*;

    #[test]
    fn parse_integer_test() {
        let mut parser = Parser::new("65").unwrap();
        assert!(matches!(parser.parse_int(), Ok(Literal::Integer(65))));
    }

    #[test]
    fn parse_indentifier_test() {
        let mut parser = Parser::new("asd").unwrap();
        assert!(parser.parse_identifier().unwrap() == "asd");
    }
}
