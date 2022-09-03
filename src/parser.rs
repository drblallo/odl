use crate::error::ParserError;
use crate::lexer::IndentLexer;
use crate::token::Token;

pub struct Parser<'a> {
    lexer: IndentLexer<'a>,
    next_token: Result<Token, ParserError>,
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
}
