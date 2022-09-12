use crate::serialization::*;
use crate::token::Span;
use core::fmt::Display;
use core::fmt::Formatter;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(i64),
    Str(String),
    Float(f64),
    Indent(String),
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        return match self {
            Literal::Integer(i) => write!(f, "Literal {}", i),
            Literal::Str(s) => write!(f, "Literal \"{}\"", s),
            Literal::Float(v) => write!(f, "Literal {}", v),
            Literal::Indent(v) => write!(f, "Literal {}", v),
        };
    }
}

impl Serializable for Literal {
    fn serialize(
        &self,
        f: &mut std::fmt::Formatter,
        ctx: &SerializationContext,
    ) -> Result<(), ::std::fmt::Error> {
        return match self {
            Literal::Integer(i) => write!(f, "{}", i),
            Literal::Str(s) => write!(f, "\"{}\"", s),
            Literal::Float(v) => write!(f, "{}", v),
            Literal::Indent(v) => write!(f, "{}", v),
        };
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryExpressionKind {
    Add,
    Sub,
    Mult,
    Div,
    Or,
    And,
    Equal,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Different,
}

impl Serializable for BinaryExpressionKind {
    fn serialize(
        &self,
        f: &mut std::fmt::Formatter,
        ctx: &SerializationContext,
    ) -> Result<(), ::std::fmt::Error> {
        match self {
            BinaryExpressionKind::Add => write!(f, "+"),
            BinaryExpressionKind::Sub => write!(f, "-"),
            BinaryExpressionKind::Mult => write!(f, "*"),
            BinaryExpressionKind::Div => write!(f, "/"),
            BinaryExpressionKind::Or => write!(f, "or"),
            BinaryExpressionKind::And => write!(f, "and"),
            BinaryExpressionKind::Equal => write!(f, "=="),
            BinaryExpressionKind::Less => write!(f, "<"),
            BinaryExpressionKind::LessEqual => write!(f, "<="),
            BinaryExpressionKind::GreaterEqual => write!(f, ">="),
            BinaryExpressionKind::Greater => write!(f, ">"),
            BinaryExpressionKind::Different => write!(f, "!="),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryExpressionKind {
    Not,
}

impl Serializable for UnaryExpressionKind {
    fn serialize(
        &self,
        f: &mut std::fmt::Formatter,
        ctx: &SerializationContext,
    ) -> Result<(), ::std::fmt::Error> {
        match self {
            UnaryExpressionKind::Not => write!(f, "-"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionEnum {
    Lit(Literal),
    Una(UnaryExpressionKind, Box<Expression>),
    Bin(BinaryExpressionKind, Box<Expression>, Box<Expression>),
}

impl Serializable for ExpressionEnum {
    fn serialize(
        &self,
        f: &mut std::fmt::Formatter,
        ctx: &SerializationContext,
    ) -> Result<(), ::std::fmt::Error> {
        return match self {
            ExpressionEnum::Lit(literal) => literal.serialize(f, ctx),
            ExpressionEnum::Una(kind, exp) => {
                kind.serialize(f, ctx)?;
                exp.serialize(f, ctx)?;
                Ok(())
            }
            ExpressionEnum::Bin(kind, lhs, rhs) => {
                write!(f, "(")?;
                lhs.serialize(f, ctx)?;
                write!(f, " ")?;
                kind.serialize(f, ctx)?;
                write!(f, " ")?;
                rhs.serialize(f, ctx)?;
                write!(f, ")")?;
                Ok(())
            }
        };
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    content: ExpressionEnum,
    span: Span,
}

impl ExpressionEnum {
    pub fn new_bin(kind: BinaryExpressionKind, lhs: Expression, rhs: Expression) -> ExpressionEnum {
        return ExpressionEnum::Bin(kind, Box::new(lhs), Box::new(rhs));
    }

    pub fn add(lhs: Expression, rhs: Expression) -> ExpressionEnum {
        return ExpressionEnum::Bin(BinaryExpressionKind::Add, Box::new(lhs), Box::new(rhs));
    }

    pub fn sub(lhs: Expression, rhs: Expression) -> ExpressionEnum {
        return ExpressionEnum::Bin(BinaryExpressionKind::Sub, Box::new(lhs), Box::new(rhs));
    }

    pub fn mult(lhs: Expression, rhs: Expression) -> ExpressionEnum {
        return ExpressionEnum::Bin(BinaryExpressionKind::Mult, Box::new(lhs), Box::new(rhs));
    }

    pub fn div(lhs: Expression, rhs: Expression) -> ExpressionEnum {
        return ExpressionEnum::Bin(BinaryExpressionKind::Div, Box::new(lhs), Box::new(rhs));
    }

    pub fn or(lhs: Expression, rhs: Expression) -> ExpressionEnum {
        return ExpressionEnum::Bin(BinaryExpressionKind::Or, Box::new(lhs), Box::new(rhs));
    }

    pub fn and(lhs: Expression, rhs: Expression) -> ExpressionEnum {
        return ExpressionEnum::Bin(BinaryExpressionKind::And, Box::new(lhs), Box::new(rhs));
    }

    pub fn equal(lhs: Expression, rhs: Expression) -> ExpressionEnum {
        return ExpressionEnum::Bin(BinaryExpressionKind::Equal, Box::new(lhs), Box::new(rhs));
    }

    pub fn different(lhs: Expression, rhs: Expression) -> ExpressionEnum {
        return ExpressionEnum::Bin(BinaryExpressionKind::Equal, Box::new(lhs), Box::new(rhs));
    }

    pub fn less(lhs: Expression, rhs: Expression) -> ExpressionEnum {
        return ExpressionEnum::Bin(BinaryExpressionKind::Less, Box::new(lhs), Box::new(rhs));
    }

    pub fn ident(lhs: String) -> ExpressionEnum {
        return ExpressionEnum::Lit(Literal::Indent(lhs.to_string()));
    }

    pub fn less_equal(lhs: Expression, rhs: Expression) -> ExpressionEnum {
        return ExpressionEnum::Bin(
            BinaryExpressionKind::LessEqual,
            Box::new(lhs),
            Box::new(rhs),
        );
    }

    pub fn greater(lhs: Expression, rhs: Expression) -> ExpressionEnum {
        return ExpressionEnum::Bin(BinaryExpressionKind::Greater, Box::new(lhs), Box::new(rhs));
    }

    pub fn greater_equal(lhs: Expression, rhs: Expression) -> ExpressionEnum {
        return ExpressionEnum::Bin(
            BinaryExpressionKind::GreaterEqual,
            Box::new(lhs),
            Box::new(rhs),
        );
    }

    pub fn new_una(kind: UnaryExpressionKind, lhs: Expression) -> ExpressionEnum {
        return ExpressionEnum::Una(kind, Box::new(lhs));
    }

    pub fn not(lhs: Expression) -> ExpressionEnum {
        return ExpressionEnum::Una(UnaryExpressionKind::Not, Box::new(lhs));
    }

    pub fn new_lit(lit: Literal) -> ExpressionEnum {
        return ExpressionEnum::Lit(lit);
    }

    pub fn str(s: String) -> ExpressionEnum {
        return ExpressionEnum::Lit(Literal::Str(s));
    }

    pub fn int(i: i64) -> ExpressionEnum {
        return ExpressionEnum::Lit(Literal::Integer(i));
    }

    pub fn float(f: f64) -> ExpressionEnum {
        return ExpressionEnum::Lit(Literal::Float(f));
    }

    pub fn arity(&self) -> usize {
        return match self {
            ExpressionEnum::Lit(_) => 0,
            ExpressionEnum::Una(_, _) => 1,
            ExpressionEnum::Bin(_, _, _) => 2,
        };
    }

    pub fn is_literal(&self) -> bool {
        return match self {
            ExpressionEnum::Lit(_) => true,
            ExpressionEnum::Una(_, _) => false,
            ExpressionEnum::Bin(_, _, _) => false,
        };
    }

    pub fn is_unary(&self) -> bool {
        return match self {
            ExpressionEnum::Lit(_) => false,
            ExpressionEnum::Una(_, _) => true,
            ExpressionEnum::Bin(_, _, _) => false,
        };
    }

    pub fn is_binary(&self) -> bool {
        return match self {
            ExpressionEnum::Lit(_) => false,
            ExpressionEnum::Una(_, _) => false,
            ExpressionEnum::Bin(_, _, _) => true,
        };
    }

    pub fn left(&self) -> Option<&Expression> {
        return match self {
            ExpressionEnum::Lit(_) => None,
            ExpressionEnum::Una(_, left) => Some(left),
            ExpressionEnum::Bin(_, left, _) => Some(left),
        };
    }

    pub fn right(&self) -> Option<&Expression> {
        return match self {
            ExpressionEnum::Lit(_) => None,
            ExpressionEnum::Una(_, _) => None,
            ExpressionEnum::Bin(_, _, right) => Some(right),
        };
    }

    pub fn literal(&self) -> Option<&Literal> {
        return match self {
            ExpressionEnum::Lit(lit) => Some(lit),
            ExpressionEnum::Una(_, _) => None,
            ExpressionEnum::Bin(_, _, _) => None,
        };
    }

    pub fn binary_kind(&self) -> Option<&BinaryExpressionKind> {
        return match self {
            ExpressionEnum::Lit(_) => None,
            ExpressionEnum::Una(_, _) => None,
            ExpressionEnum::Bin(kind, _, _) => Some(kind),
        };
    }

    pub fn unary_kind(&self) -> Option<&UnaryExpressionKind> {
        return match self {
            ExpressionEnum::Lit(_) => None,
            ExpressionEnum::Una(kind, _) => Some(kind),
            ExpressionEnum::Bin(_, _, _) => None,
        };
    }
}

impl Expression {
    pub fn new_bin(
        kind: BinaryExpressionKind,
        lhs: Expression,
        rhs: Expression,
        span: Span,
    ) -> Expression {
        let content = ExpressionEnum::new_bin(kind, lhs, rhs);
        return Expression { content, span };
    }

    pub fn add(lhs: Expression, rhs: Expression, span: Span) -> Expression {
        let content = ExpressionEnum::add(lhs, rhs);
        return Expression { content, span };
    }

    pub fn sub(lhs: Expression, rhs: Expression, span: Span) -> Expression {
        let content = ExpressionEnum::sub(lhs, rhs);
        return Expression { content, span };
    }

    pub fn mult(lhs: Expression, rhs: Expression, span: Span) -> Expression {
        let content = ExpressionEnum::mult(lhs, rhs);
        return Expression { content, span };
    }

    pub fn div(lhs: Expression, rhs: Expression, span: Span) -> Expression {
        let content = ExpressionEnum::div(lhs, rhs);
        return Expression { content, span };
    }

    pub fn or(lhs: Expression, rhs: Expression, span: Span) -> Expression {
        let content = ExpressionEnum::or(lhs, rhs);
        return Expression { content, span };
    }

    pub fn and(lhs: Expression, rhs: Expression, span: Span) -> Expression {
        let content = ExpressionEnum::and(lhs, rhs);
        return Expression { content, span };
    }

    pub fn equal(lhs: Expression, rhs: Expression, span: Span) -> Expression {
        let content = ExpressionEnum::equal(lhs, rhs);
        return Expression { content, span };
    }

    pub fn different(lhs: Expression, rhs: Expression, span: Span) -> Expression {
        let content = ExpressionEnum::different(lhs, rhs);
        return Expression { content, span };
    }

    pub fn less(lhs: Expression, rhs: Expression, span: Span) -> Expression {
        let content = ExpressionEnum::less(lhs, rhs);
        return Expression { content, span };
    }

    pub fn ident(lhs: String, span: Span) -> Expression {
        let content = ExpressionEnum::ident(lhs.to_string());
        return Expression { content, span };
    }

    pub fn less_equal(lhs: Expression, rhs: Expression, span: Span) -> Expression {
        let content = ExpressionEnum::less_equal(lhs, rhs);
        return Expression { content, span };
    }

    pub fn greater(lhs: Expression, rhs: Expression, span: Span) -> Expression {
        let content = ExpressionEnum::greater(lhs, rhs);
        return Expression { content, span };
    }

    pub fn greater_equal(lhs: Expression, rhs: Expression, span: Span) -> Expression {
        let content = ExpressionEnum::greater_equal(lhs, rhs);
        return Expression { content, span };
    }

    pub fn new_una(kind: UnaryExpressionKind, lhs: Expression, span: Span) -> Expression {
        let content = ExpressionEnum::new_una(kind, lhs);
        return Expression { content, span };
    }

    pub fn not(lhs: Expression, span: Span) -> Expression {
        let content = ExpressionEnum::not(lhs);
        return Expression { content, span };
    }

    pub fn new_lit(lit: Literal, span: Span) -> Expression {
        let content = ExpressionEnum::new_lit(lit);
        return Expression { content, span };
    }

    pub fn str(s: String, span: Span) -> Expression {
        let content = ExpressionEnum::str(s);
        return Expression { content, span };
    }

    pub fn int(i: i64, span: Span) -> Expression {
        let content = ExpressionEnum::int(i);
        return Expression { content, span };
    }

    pub fn float(f: f64, span: Span) -> Expression {
        let content = ExpressionEnum::float(f);
        return Expression { content, span };
    }

    pub fn arity(&self) -> usize {
        return self.content.arity();
    }

    pub fn is_literal(&self) -> bool {
        return self.content.is_literal();
    }

    pub fn is_unary(&self) -> bool {
        return self.content.is_unary();
    }

    pub fn is_binary(&self) -> bool {
        return self.content.is_binary();
    }

    pub fn left(&self) -> Option<&Expression> {
        return self.content.left();
    }

    pub fn right(&self) -> Option<&Expression> {
        return self.content.right();
    }

    pub fn literal(&self) -> Option<&Literal> {
        return self.content.literal();
    }

    pub fn binary_kind(&self) -> Option<&BinaryExpressionKind> {
        return self.content.binary_kind();
    }

    pub fn unary_kind(&self) -> Option<&UnaryExpressionKind> {
        return self.content.unary_kind();
    }

    pub fn span(&self) -> Span {
        return self.span.clone();
    }

    pub fn set_span(&mut self, span: Span) {
        return self.span = span;
    }
}

impl Serializable for Expression {
    fn serialize(
        &self,
        f: &mut std::fmt::Formatter,
        ctx: &SerializationContext,
    ) -> Result<(), ::std::fmt::Error> {
        return self.content.serialize(f, ctx);
    }
}
