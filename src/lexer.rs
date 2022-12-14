use crate::error::ParserError;
use crate::token::*;
use plex::lexer;

lexer! {
    fn next_token(text: 'a) -> TokenKind;

    r#"[ \t\r]"# => TokenKind::Whitespace(1),
    r#"[\n]+"# => TokenKind::EndLine,
    // "C++-style" comments (// ...)
    r#"#[^\n]*"# => TokenKind::Comment,

    r#"const"# => TokenKind::Const,
    r#"opt"# => TokenKind::Opt,
    r#"alt"# => TokenKind::Alt,

    r#"or"# => TokenKind::Or,
    r#"and"# => TokenKind::And,
    r#"=="# => TokenKind::Equals,
    r#"!="# => TokenKind::Different,

    r#"<="# => TokenKind::LessEqual,
    r#"<"# => TokenKind::Less,
    r#">="# => TokenKind::GreaterEqual,
    r#">"# => TokenKind::Greater,


    r#"[0-9]+"# => {
        if let Ok(i) = text.parse() {
            TokenKind::Integer(i)
        } else {
            panic!("integer {} is out of range", text)
        }
    }

    r#"[a-zA-Z_][a-zA-Z0-9_]*"# => TokenKind::Ident(text.to_owned()),

    r#"\+"# => TokenKind::Plus,
    r#"-"# => TokenKind::Minus,
    r#"\*"# => TokenKind::Star,
    r#"/"# => TokenKind::Slash,
    r#"\("# => TokenKind::LParen,
    r#"\)"# => TokenKind::RParen,
    r#";"# => TokenKind::Semi,
    r#"="# => TokenKind::Assign,

    r#"."# => panic!("unexpected character: {}", text),
}

pub struct Lexer<'a> {
    original: &'a str,
    remaining: &'a str,
    location: SourceLocation,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Lexer<'a> {
        Lexer {
            original: s,
            remaining: s,
            location: SourceLocation { row: 0, column: 0 },
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        loop {
            let token = if let Some((tok, new_remaining)) = next_token(self.remaining) {
                let begin = self.original.len() - self.remaining.len();
                let end = self.original.len() - new_remaining.len();
                self.remaining = new_remaining;

                let lo = self.location.clone();

                for i in begin..end {
                    match self.original.chars().nth(i).unwrap() {
                        '\n' => {
                            self.location.row = self.location.row + 1;
                            self.location.column = 0;
                        }
                        _ => {
                            self.location.column = self.location.column + 1;
                        }
                    }
                }

                let hi = self.location.clone();
                Some(Token::new(tok, lo, hi))
            } else {
                return None;
            };

            if let Some(Token {
                kind: TokenKind::Comment,
                span: _span,
            }) = token
            {
                continue;
            }

            return token;
        }
    }
}

pub struct IndentLexer<'a> {
    lexer: Lexer<'a>,
    next_token: Option<Token>,
    current_token: Option<Token>,
    indentation_stack: Vec<i64>,
    start_of_line: bool,
    deindent_to_emit: Vec<Token>,
}

impl<'a> IndentLexer<'a> {
    fn advance_impl(&mut self) {
        let expelled_token = self.current_token.clone();
        self.current_token = self.next_token.clone();
        self.next_token = self.lexer.next();

        while let Some(value) = self
            .current_token
            .as_ref()
            .zip(self.next_token.as_ref())
            .and_then(|(a, b)| a.try_merge_whitespace(&b))
        {
            self.next_token = Some(value);
            self.current_token = self.next_token.clone();
            self.next_token = self.lexer.next();
        }

        self.start_of_line = match expelled_token {
            Some(Token {
                kind: TokenKind::EndLine,
                span: _span,
            }) => true,
            None => true,
            _ => false,
        }
    }

    pub fn new(s: &'a str) -> IndentLexer<'a> {
        let mut to_return = IndentLexer {
            lexer: Lexer::new(s),
            next_token: None,
            current_token: None,
            indentation_stack: Vec::new(),
            start_of_line: true,
            deindent_to_emit: Vec::new(),
        };

        to_return.next_token = to_return.lexer.next();
        to_return.advance_impl();
        return to_return;
    }

    fn next_with_whitespace(&mut self) -> Result<Token, ParserError> {
        if let Err(error) = self.handle_indent() {
            self.advance_impl();
            return Err(error);
        }
        if let Some(token) = self.deindent_to_emit.pop() {
            return Ok(token);
        }

        let to_return = self.current_token.clone();
        self.advance_impl();

        if to_return.is_none() && !self.indentation_stack.is_empty() {
            let to_emit = Token {
                kind: TokenKind::Deindent,
                span: Span {
                    lo: self.lexer.location,
                    hi: self.lexer.location,
                },
            };
            self.indentation_stack.pop();
            return Ok(to_emit);
        }

        return match to_return {
            Some(value) => Ok(value),
            None => Err(ParserError::new_end_of_token_stream()),
        };
    }

    pub fn handle_indent(&mut self) -> Result<(), ParserError> {
        if !self.start_of_line || self.current_token.is_none() {
            return Ok(());
        }

        let Token { kind, span } = self.current_token.as_ref().unwrap();

        let current_white_space = match kind {
            TokenKind::Whitespace(a) => a.clone(),
            _ => 0,
        };

        let zero: i64 = 0;
        if self.indentation_stack.last().unwrap_or(&zero) == &current_white_space {
            return Ok(());
        }

        if self.indentation_stack.last().unwrap_or(&zero) < &current_white_space {
            let to_emit = Token {
                kind: TokenKind::Indent,
                span: span.clone(),
            };

            self.deindent_to_emit.push(to_emit);
            self.indentation_stack.push(current_white_space);
            self.start_of_line = false;
            return Ok(());
        }

        loop {
            let indent = self.indentation_stack.pop().unwrap_or(zero);
            if indent < current_white_space {
                return Err(ParserError::new_indentation_miss_match(
                    span.clone(),
                    indent,
                    current_white_space,
                ));
            } else if indent == current_white_space {
                if indent != 0 {
                    self.indentation_stack.push(current_white_space);
                }
                return Ok(());
            }

            let to_emit = Token {
                kind: TokenKind::Deindent,
                span: span.clone(),
            };

            self.deindent_to_emit.push(to_emit);
            self.start_of_line = false;
        }
    }

    pub fn next_token(&mut self) -> Result<Token, ParserError> {
        loop {
            let token = self.next_with_whitespace()?;
            if token.is_whitespace() || token.kind == TokenKind::EndLine {
                continue;
            }

            return Ok(token);
        }
    }
}

impl<'a> Iterator for IndentLexer<'a> {
    type Item = Result<Token, ParserError>;

    fn next(&mut self) -> Option<Result<Token, ParserError>> {
        let token = self.next_token();
        match token {
            Ok(token) => return Some(Ok(token)),
            Err(content) => {
                if content.is_end_of_token_stream() {
                    return None;
                } else {
                    return Some(Err(content));
                }
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::error::ParserError;
    use crate::lexer::IndentLexer;
    use crate::token::*;

    fn token_kind(option: &Option<Result<Token, ParserError>>) -> TokenKind {
        assert!(option.is_some());
        let maybe_error = option.as_ref().unwrap();
        assert!(maybe_error.is_ok());
        let token = maybe_error.as_ref().unwrap();
        return token.kind.clone();
    }

    fn token_span(option: &Option<Result<Token, ParserError>>) -> Span {
        assert!(option.is_some());
        let maybe_error = option.as_ref().unwrap();
        assert!(maybe_error.is_ok());
        let token = maybe_error.as_ref().unwrap();
        return token.span.clone();
    }

    #[test]
    fn identifier_test() {
        let mut lexer = IndentLexer::new("hey");
        let kind = token_kind(&lexer.next());
        assert_eq!(kind, TokenKind::Ident("hey".to_owned()));
        assert!(lexer.next().is_none());
    }

    #[test]
    fn span_identifier_test() {
        let mut lexer = IndentLexer::new("hey");
        let span = token_span(&lexer.next());
        assert_eq!(span.lo, SourceLocation { row: 0, column: 0 });
        assert_eq!(span.hi, SourceLocation { row: 0, column: 3 });
        assert!(lexer.next().is_none());
    }

    #[test]
    fn comment_test() {
        let mut lexer = IndentLexer::new("asd #hey\n");
        assert_eq!(
            token_kind(&lexer.next()),
            TokenKind::Ident("asd".to_owned())
        );
        assert!(lexer.next().is_none());
    }

    #[test]
    fn indent_test() {
        let mut lexer = IndentLexer::new(" asd\n  asd\n asd\nasd\n");
        let asd_token = TokenKind::Ident("asd".to_owned());
        assert_eq!(token_kind(&lexer.next()), TokenKind::Indent);
        assert_eq!(token_kind(&lexer.next()), asd_token);
        assert_eq!(token_kind(&lexer.next()), TokenKind::Indent);
        assert_eq!(token_kind(&lexer.next()), asd_token);
        assert_eq!(token_kind(&lexer.next()), TokenKind::Deindent);
        assert_eq!(token_kind(&lexer.next()), asd_token);
        assert_eq!(token_kind(&lexer.next()), TokenKind::Deindent);
        assert_eq!(token_kind(&lexer.next()), asd_token);
        assert!(lexer.next().is_none());
    }

    #[test]
    fn fail_indent_test() {
        let mut lexer = IndentLexer::new("  asd\n asd\n");
        let asd_token = TokenKind::Ident("asd".to_owned());
        assert_eq!(token_kind(&lexer.next()), TokenKind::Indent);
        assert_eq!(token_kind(&lexer.next()), asd_token);
        assert!(lexer.next().unwrap().is_err());
    }

    #[test]
    fn assing_test() {
        let mut lexer = IndentLexer::new(" = ");
        assert_eq!(token_kind(&lexer.next()), TokenKind::Indent);
        assert_eq!(token_kind(&lexer.next()), TokenKind::Assign);
        assert_eq!(token_kind(&lexer.next()), TokenKind::Deindent);
        assert!(lexer.next().is_none());
    }

    #[test]
    fn constant_test() {
        let mut lexer = IndentLexer::new("const asd\n rasd = 4\n\n");
        assert_eq!(token_kind(&lexer.next()), TokenKind::Const);
        assert_eq!(
            token_kind(&lexer.next()),
            TokenKind::Ident("asd".to_owned())
        );
        assert_eq!(token_kind(&lexer.next()), TokenKind::Indent);
        assert_eq!(
            token_kind(&lexer.next()),
            TokenKind::Ident("rasd".to_owned())
        );
        assert_eq!(token_kind(&lexer.next()), TokenKind::Assign);
        assert_eq!(token_kind(&lexer.next()), TokenKind::Integer(4));
        assert_eq!(token_kind(&lexer.next()), TokenKind::Deindent);
        assert!(lexer.next().is_none());
    }

    #[test]
    fn indent_test_2() {
        let mut lexer = IndentLexer::new("\n \n\n");
        assert_eq!(token_kind(&lexer.next()), TokenKind::Indent);
        assert_eq!(token_kind(&lexer.next()), TokenKind::Deindent);
        assert!(lexer.next().is_none());
    }
}
