use crate::token::Span;
use core::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(i64),
    Str(String),
    Float(f64),
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        return match self {
            Literal::Integer(i) => write!(f, "Literal {}", i),
            Literal::Str(s) => write!(f, "Literal \"{}\"", s),
            Literal::Float(v) => write!(f, "Literal {}", v),
        };
    }
}

pub enum BinaryExpressionKind {
    Add,
    Sub,
    Mult,
    Div,
}

pub enum UnaryExpressionKind {
    Not,
}

pub enum Expression {
    Lit(Literal),
    Una(UnaryExpressionKind, Box<Expression>),
    Bin(BinaryExpressionKind, Box<Expression>, Box<Expression>),
}

impl Expression {
    fn new_bin(kind: BinaryExpressionKind, lhs: Expression, rhs: Expression) -> Expression {
        return Expression::Bin(kind, Box::new(lhs), Box::new(rhs));
    }

    fn add(lhs: Expression, rhs: Expression) -> Expression {
        return Expression::Bin(BinaryExpressionKind::Add, Box::new(lhs), Box::new(rhs));
    }

    fn sub(lhs: Expression, rhs: Expression) -> Expression {
        return Expression::Bin(BinaryExpressionKind::Sub, Box::new(lhs), Box::new(rhs));
    }

    fn mult(lhs: Expression, rhs: Expression) -> Expression {
        return Expression::Bin(BinaryExpressionKind::Mult, Box::new(lhs), Box::new(rhs));
    }

    fn div(lhs: Expression, rhs: Expression) -> Expression {
        return Expression::Bin(BinaryExpressionKind::Div, Box::new(lhs), Box::new(rhs));
    }

    fn new_una(kind: UnaryExpressionKind, lhs: Expression) -> Expression {
        return Expression::Una(kind, Box::new(lhs));
    }

    fn not(lhs: Expression) -> Expression {
        return Expression::Una(UnaryExpressionKind::Not, Box::new(lhs));
    }

    fn new_lit(lit: Literal) -> Expression {
        return Expression::Lit(lit);
    }

    fn str(s: String) -> Expression {
        return Expression::Lit(Literal::Str(s));
    }

    fn int(i: i64) -> Expression {
        return Expression::Lit(Literal::Integer(i));
    }

    fn float(f: f64) -> Expression {
        return Expression::Lit(Literal::Float(f));
    }

    fn arity(&self) -> usize {
        return match self {
            Expression::Lit(_) => 0,
            Expression::Una(_, _) => 1,
            Expression::Bin(_, _, _) => 2,
        };
    }

    fn is_literal(&self) -> bool {
        return match self {
            Expression::Lit(_) => true,
            Expression::Una(_, _) => false,
            Expression::Bin(_, _, _) => false,
        };
    }

    fn is_unary(&self) -> bool {
        return match self {
            Expression::Lit(_) => false,
            Expression::Una(_, _) => true,
            Expression::Bin(_, _, _) => false,
        };
    }

    fn is_binary(&self) -> bool {
        return match self {
            Expression::Lit(_) => false,
            Expression::Una(_, _) => false,
            Expression::Bin(_, _, _) => true,
        };
    }

    fn left(&self) -> Option<&Expression> {
        return match self {
            Expression::Lit(_) => None,
            Expression::Una(_, left) => Some(left),
            Expression::Bin(_, left, _) => Some(left),
        };
    }

    fn right(&self) -> Option<&Expression> {
        return match self {
            Expression::Lit(_) => None,
            Expression::Una(_, _) => None,
            Expression::Bin(_, _, right) => Some(right),
        };
    }

    fn literal(&self) -> Option<&Literal> {
        return match self {
            Expression::Lit(lit) => Some(lit),
            Expression::Una(_, _) => None,
            Expression::Bin(_, _, _) => None,
        };
    }

    fn binary_kind(&self) -> Option<&BinaryExpressionKind> {
        return match self {
            Expression::Lit(_) => None,
            Expression::Una(_, _) => None,
            Expression::Bin(kind, _, _) => Some(kind),
        };
    }

    fn unary_kind(&self) -> Option<&UnaryExpressionKind> {
        return match self {
            Expression::Lit(_) => None,
            Expression::Una(kind, _) => Some(kind),
            Expression::Bin(_, _, _) => None,
        };
    }
}
