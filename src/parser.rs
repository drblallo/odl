use crate::alternative::*;
use crate::constant::*;
use crate::declaration::*;
use crate::error::ParserError;
use crate::expression::*;
use crate::lexer::IndentLexer;
use crate::option::*;
use crate::token::*;

pub struct Parser<'a> {
    lexer: IndentLexer<'a>,
    current_token: Option<Result<Token, ParserError>>,
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
macro_rules! peek {
    ($parser: expr, $pattern:pat) => {
        $parser.peek().is_some() && matches!($parser.peek().unwrap().kind, $pattern)
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
        let current_token = None;
        return Ok(Parser {
            lexer,
            current_token,
            next_token,
        });
    }

    fn peek(&self) -> Option<Token> {
        return match self.next_token.as_ref() {
            Err(_value) => None,
            Ok(value) => Some(value.clone()),
        };
    }

    fn current_span(&self) -> Result<Span, ParserError> {
        return match self.current_token.as_ref() {
            None => Ok(Span::new()),
            Some(Err(err)) => Err(err.clone()),
            Some(Ok(token)) => Ok(token.span.clone()),
        };
    }

    fn current(&self) -> Result<Token, ParserError> {
        return self.current_token.clone().unwrap();
    }

    fn next(&mut self) -> Result<Token, ParserError> {
        self.current_token = Some(self.next_token.clone());
        self.next_token = self.lexer.next_token();
        return self.current_token.clone().unwrap();
    }

    fn int(&mut self) -> Result<Literal, ParserError> {
        let token = expect!(self, TokenKind::Integer(_i));
        return Ok(Literal::Integer(token.get_int().unwrap()));
    }

    fn identifier(&mut self) -> Result<String, ParserError> {
        let token = expect!(self, TokenKind::Ident(_i));
        return Ok(token.get_identifier().unwrap());
    }

    fn primary_expression(&mut self) -> Result<Expression, ParserError> {
        let start = self.current_span()?;
        if accept!(self, TokenKind::Ident(_)) {
            let lhs = self.current().unwrap().get_identifier().unwrap();
            return Ok(Expression::ident(lhs, start.merge(&self.current_span()?)));
        }
        if accept!(self, TokenKind::Integer(_)) {
            let lhs = self.current().unwrap().get_int().unwrap();
            return Ok(Expression::int(lhs, start.merge(&self.current_span()?)));
        }
        if accept!(self, TokenKind::LParen) {
            let mut lhs = self.expression()?;
            expect!(self, TokenKind::RParen);
            lhs.set_span(start.merge(&self.current_span()?));
            return Ok(lhs);
        }

        return self.primary_expression();
    }

    fn unary_expression(&mut self) -> Result<Expression, ParserError> {
        let start = self.current_span()?;
        if accept!(self, TokenKind::Minus) {
            let lhs = self.unary_expression()?;
            return Ok(Expression::not(lhs, start.merge(&self.current_span()?)));
        } else if accept!(self, TokenKind::Plus) {
            let lhs = self.unary_expression()?;
            return Ok(lhs);
        }

        return self.primary_expression();
    }

    fn multiplicative_expression(&mut self) -> Result<Expression, ParserError> {
        let start = self.current_span()?;
        let lhs = self.unary_expression()?;
        if accept!(self, TokenKind::Star) {
            let rhs = self.multiplicative_expression()?;
            return Ok(Expression::mult(
                lhs,
                rhs,
                start.merge(&self.current_span()?),
            ));
        } else if accept!(self, TokenKind::Slash) {
            let rhs = self.multiplicative_expression()?;
            return Ok(Expression::div(
                lhs,
                rhs,
                start.merge(&self.current_span()?),
            ));
        }

        return Ok(lhs);
    }

    fn additive_expression(&mut self) -> Result<Expression, ParserError> {
        let start = self.current_span()?;
        let lhs = self.multiplicative_expression()?;
        if accept!(self, TokenKind::Plus) {
            let rhs = self.additive_expression()?;
            return Ok(Expression::add(
                lhs,
                rhs,
                start.merge(&self.current_span()?),
            ));
        } else if accept!(self, TokenKind::Minus) {
            let rhs = self.additive_expression()?;
            return Ok(Expression::sub(
                lhs,
                rhs,
                start.merge(&self.current_span()?),
            ));
        }

        return Ok(lhs);
    }

    fn relational_expression(&mut self) -> Result<Expression, ParserError> {
        let start = self.current_span()?;
        let lhs = self.additive_expression()?;
        if accept!(self, TokenKind::Less) {
            let rhs = self.relational_expression()?;
            return Ok(Expression::less(
                lhs,
                rhs,
                start.merge(&self.current_span()?),
            ));
        } else if accept!(self, TokenKind::LessEqual) {
            let rhs = self.relational_expression()?;
            return Ok(Expression::less_equal(
                lhs,
                rhs,
                start.merge(&self.current_span()?),
            ));
        } else if accept!(self, TokenKind::Greater) {
            let rhs = self.relational_expression()?;
            return Ok(Expression::greater(
                lhs,
                rhs,
                start.merge(&self.current_span()?),
            ));
        } else if accept!(self, TokenKind::GreaterEqual) {
            let rhs = self.relational_expression()?;
            return Ok(Expression::greater_equal(
                lhs,
                rhs,
                start.merge(&self.current_span()?),
            ));
        }
        return Ok(lhs);
    }

    fn equal_expression(&mut self) -> Result<Expression, ParserError> {
        let start = self.current_span()?;
        let lhs = self.relational_expression()?;
        if accept!(self, TokenKind::Equals) {
            let rhs = self.equal_expression()?;
            return Ok(Expression::equal(
                lhs,
                rhs,
                start.merge(&self.current_span()?),
            ));
        } else if accept!(self, TokenKind::Different) {
            let rhs = self.equal_expression()?;
            return Ok(Expression::different(
                lhs,
                rhs,
                start.merge(&self.current_span()?),
            ));
        }

        return Ok(lhs);
    }

    fn and_expression(&mut self) -> Result<Expression, ParserError> {
        let start = self.current_span()?;
        let lhs = self.equal_expression()?;
        if !accept!(self, TokenKind::And) {
            return Ok(lhs);
        }
        let rhs = self.and_expression()?;
        return Ok(Expression::and(
            lhs,
            rhs,
            start.merge(&self.current_span()?),
        ));
    }

    fn expression(&mut self) -> Result<Expression, ParserError> {
        let start = self.current_span()?;
        let lhs = self.and_expression()?;
        if !accept!(self, TokenKind::Or) {
            return Ok(lhs);
        }
        let rhs = self.expression()?;
        return Ok(Expression::or(lhs, rhs, start.merge(&self.current_span()?)));
    }

    fn constant_body(&mut self) -> Result<ConstantDeclaration, ParserError> {
        let start = self.current_span()?;
        let name = self.identifier()?;

        if accept!(self, TokenKind::Assign) {
            let initializer = self.expression()?;
            return Ok(ConstantDeclaration::new_direct(
                name,
                initializer,
                start.merge(&self.current_span()?),
            ));
        }

        expect!(self, TokenKind::Indent);

        let mut children = Vec::new();
        while !accept!(self, TokenKind::Deindent) {
            children.push(self.constant_body()?);
        }

        let mut x = ConstantDeclaration::new(name, start.merge(&self.current_span()?));
        *x.get_fields_mut().unwrap() = children;
        return Ok(x);
    }

    pub fn constant_declaration(&mut self) -> Result<ConstantDeclaration, ParserError> {
        let start = self.current_span()?;
        expect!(self, TokenKind::Const);
        let mut constant = self.constant_body()?;
        constant.set_span(start.merge(&self.current_span()?));
        return Ok(constant);
    }

    pub fn option_field_declaration(&mut self) -> Result<OptionField, ParserError> {
        if self.peek().map_or(false, |x| x.kind == TokenKind::Const) {
            let decl = self.constant_declaration()?;
            return Ok(OptionField::Const(decl));
        }
        return Ok(OptionField::SubOption(self.option_declaration_body()?));
    }

    pub fn option_declaration_body(&mut self) -> Result<OptionDeclaration, ParserError> {
        let start = self.current_span()?;
        let name = self.identifier()?;

        if !accept!(self, TokenKind::Indent) {
            return Ok(OptionDeclaration::new(
                name,
                start.merge(&self.current_span()?),
            ));
        }

        let mut declarations = Vec::new();
        while !accept!(self, TokenKind::Deindent) {
            declarations.push(self.option_field_declaration()?);
        }

        let mut decl = OptionDeclaration::new(name, start.merge(&self.current_span()?));
        *decl.get_fields_mut() = declarations;
        return Ok(decl);
    }

    pub fn option_declaration(&mut self) -> Result<OptionDeclaration, ParserError> {
        let start = self.current_span()?;
        expect!(self, TokenKind::Opt);
        let mut declaration = self.option_declaration_body()?;
        declaration.set_span(start.merge(&self.current_span()?));
        return Ok(declaration);
    }

    pub fn alternative_declaration_body(&mut self) -> Result<AlternativeDeclaration, ParserError> {
        let start = self.current_span()?;
        let name = self.identifier()?;

        if !accept!(self, TokenKind::Indent) {
            return Ok(AlternativeDeclaration::new(
                name,
                start.merge(&self.current_span()?),
            ));
        }

        let mut declarations = Vec::new();
        while !accept!(self, TokenKind::Deindent) {
            declarations.push(self.option_declaration_body()?);
        }

        let mut decl = AlternativeDeclaration::new(name, start.merge(&self.current_span()?));
        *decl.get_fields_mut() = declarations;
        return Ok(decl);
    }

    pub fn alternative_declaration(&mut self) -> Result<AlternativeDeclaration, ParserError> {
        let start = self.current_span()?;
        expect!(self, TokenKind::Alt);
        let mut declaration = self.alternative_declaration_body()?;
        declaration.set_span(start.merge(&self.current_span()?));
        return Ok(declaration);
    }

    pub fn declaration(&mut self) -> Result<Declaration, ParserError> {
        if peek!(self, TokenKind::Const) {
            let decl = self.constant_declaration()?;
            return Ok(Declaration::Const(decl));
        }
        if peek!(self, TokenKind::Opt) {
            let decl = self.option_declaration()?;
            return Ok(Declaration::Opt(decl));
        }
        if peek!(self, TokenKind::Alt) {
            let decl = self.alternative_declaration()?;
            return Ok(Declaration::Alt(decl));
        }
        return Err(ParserError::new_unexpected_token(self.current()?));
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::*;

    #[test]
    fn integer_test() {
        let mut parser = Parser::new("65").unwrap();
        assert!(matches!(parser.int(), Ok(Literal::Integer(65))));
    }

    #[test]
    fn indentifier_test() {
        let mut parser = Parser::new("asd").unwrap();
        assert!(parser.identifier().unwrap() == "asd");
    }

    #[test]
    fn int_expression() {
        let mut parser = Parser::new("43 + 53").unwrap();
        let maybe_expression = parser.expression();
        assert!(maybe_expression.is_ok());
        let expression = maybe_expression.unwrap();
        assert!(expression.is_binary());
        assert_eq!(
            *expression.binary_kind().unwrap(),
            BinaryExpressionKind::Add
        );
        let rhs = expression.right();
        let lhs = expression.left();
        assert!(rhs.unwrap().is_literal());
        assert!(lhs.unwrap().is_literal());
        assert_eq!(*lhs.unwrap().literal().unwrap(), Literal::Integer(43));
        assert_eq!(*rhs.unwrap().literal().unwrap(), Literal::Integer(53));
        assert_eq!(expression.span().lo.column, 0);
        assert_eq!(expression.span().hi.column, 7);
    }

    #[test]
    fn equal_expression() {
        let mut parser = Parser::new("43 == 53").unwrap();
        let maybe_expression = parser.expression();
        assert!(maybe_expression.is_ok());
        let expression = maybe_expression.unwrap();
        assert!(expression.is_binary());
        assert_eq!(
            *expression.binary_kind().unwrap(),
            BinaryExpressionKind::Equal
        );
        let rhs = expression.right();
        let lhs = expression.left();
        assert!(rhs.unwrap().is_literal());
        assert!(lhs.unwrap().is_literal());
        assert_eq!(*lhs.unwrap().literal().unwrap(), Literal::Integer(43));
        assert_eq!(*rhs.unwrap().literal().unwrap(), Literal::Integer(53));
        assert_eq!(expression.span().lo.column, 0);
        assert_eq!(expression.span().hi.column, 8);
    }

    #[test]
    fn and_expression() {
        let mut parser = Parser::new("43 and 53").unwrap();
        let maybe_expression = parser.expression();
        assert!(maybe_expression.is_ok());
        let expression = maybe_expression.unwrap();
        assert!(expression.is_binary());
        assert_eq!(
            *expression.binary_kind().unwrap(),
            BinaryExpressionKind::And
        );
        let rhs = expression.right();
        let lhs = expression.left();
        assert!(rhs.unwrap().is_literal());
        assert!(lhs.unwrap().is_literal());
        assert_eq!(*lhs.unwrap().literal().unwrap(), Literal::Integer(43));
        assert_eq!(*rhs.unwrap().literal().unwrap(), Literal::Integer(53));
        assert_eq!(expression.span().lo.column, 0);
        assert_eq!(expression.span().hi.column, 9);
    }

    #[test]
    fn les_expression() {
        let mut parser = Parser::new("43 < 53").unwrap();
        let maybe_expression = parser.expression();
        assert!(maybe_expression.is_ok());
        let expression = maybe_expression.unwrap();
        assert!(expression.is_binary());
        assert_eq!(
            *expression.binary_kind().unwrap(),
            BinaryExpressionKind::Less
        );
        let rhs = expression.right();
        let lhs = expression.left();
        assert!(rhs.unwrap().is_literal());
        assert!(lhs.unwrap().is_literal());
        assert_eq!(*lhs.unwrap().literal().unwrap(), Literal::Integer(43));
        assert_eq!(*rhs.unwrap().literal().unwrap(), Literal::Integer(53));
        assert_eq!(expression.span().lo.column, 0);
        assert_eq!(expression.span().hi.column, 7);
    }

    #[test]
    fn par_expression() {
        let mut parser = Parser::new("(43 >= 53)").unwrap();
        let maybe_expression = parser.expression();
        assert!(maybe_expression.is_ok());
        let expression = maybe_expression.unwrap();
        assert!(expression.is_binary());
        assert_eq!(
            *expression.binary_kind().unwrap(),
            BinaryExpressionKind::GreaterEqual
        );
        let rhs = expression.right();
        let lhs = expression.left();
        assert!(rhs.unwrap().is_literal());
        assert!(lhs.unwrap().is_literal());
        assert_eq!(*lhs.unwrap().literal().unwrap(), Literal::Integer(43));
        assert_eq!(*rhs.unwrap().literal().unwrap(), Literal::Integer(53));
        assert_eq!(expression.span().lo.column, 0);
        assert_eq!(expression.span().hi.column, 10);
    }

    #[test]
    fn constant_declaration() {
        let mut parser = Parser::new("const asd = (43 >= 53)").unwrap();
        let maybe_declaration = parser.constant_declaration();
        assert!(maybe_declaration.is_ok());
        let declaration = maybe_declaration.unwrap();
        assert_eq!(declaration.name(), "asd");
        let expression = declaration.get_initializer().unwrap();
        assert!(expression.is_binary());
        assert_eq!(
            *expression.binary_kind().unwrap(),
            BinaryExpressionKind::GreaterEqual
        );
        let rhs = expression.right();
        let lhs = expression.left();
        assert!(rhs.unwrap().is_literal());
        assert!(lhs.unwrap().is_literal());
        assert_eq!(*lhs.unwrap().literal().unwrap(), Literal::Integer(43));
        assert_eq!(*rhs.unwrap().literal().unwrap(), Literal::Integer(53));
    }

    #[test]
    fn constant_declaration_multiline() {
        let mut parser = Parser::new("const asd\n rasd = 4\n\n").unwrap();
        let maybe_declaration = parser.constant_declaration();
        assert!(maybe_declaration.is_ok());
        let declaration = maybe_declaration.unwrap();
        assert_eq!(declaration.name(), "asd");
        let field = declaration.get_field(0);
        assert_eq!(field.unwrap().name(), "rasd");
    }

    #[test]
    fn empty_alternative_declaration() {
        let mut parser = Parser::new("alt asd").unwrap();
        let maybe_declaration = parser.alternative_declaration();
        assert!(maybe_declaration.is_ok());
        let declaration = maybe_declaration.unwrap();
        assert_eq!(declaration.name(), "asd");
    }
}
