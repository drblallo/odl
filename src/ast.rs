use crate::token::Span;
use core::fmt::Display;

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

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryExpressionKind {
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Lit(Literal),
    Una(UnaryExpressionKind, Box<Expression>),
    Bin(BinaryExpressionKind, Box<Expression>, Box<Expression>),
}

impl Expression {
    pub fn new_bin(kind: BinaryExpressionKind, lhs: Expression, rhs: Expression) -> Expression {
        return Expression::Bin(kind, Box::new(lhs), Box::new(rhs));
    }

    pub fn add(lhs: Expression, rhs: Expression) -> Expression {
        return Expression::Bin(BinaryExpressionKind::Add, Box::new(lhs), Box::new(rhs));
    }

    pub fn sub(lhs: Expression, rhs: Expression) -> Expression {
        return Expression::Bin(BinaryExpressionKind::Sub, Box::new(lhs), Box::new(rhs));
    }

    pub fn mult(lhs: Expression, rhs: Expression) -> Expression {
        return Expression::Bin(BinaryExpressionKind::Mult, Box::new(lhs), Box::new(rhs));
    }

    pub fn div(lhs: Expression, rhs: Expression) -> Expression {
        return Expression::Bin(BinaryExpressionKind::Div, Box::new(lhs), Box::new(rhs));
    }

    pub fn or(lhs: Expression, rhs: Expression) -> Expression {
        return Expression::Bin(BinaryExpressionKind::Or, Box::new(lhs), Box::new(rhs));
    }

    pub fn and(lhs: Expression, rhs: Expression) -> Expression {
        return Expression::Bin(BinaryExpressionKind::And, Box::new(lhs), Box::new(rhs));
    }

    pub fn equal(lhs: Expression, rhs: Expression) -> Expression {
        return Expression::Bin(BinaryExpressionKind::Equal, Box::new(lhs), Box::new(rhs));
    }

    pub fn different(lhs: Expression, rhs: Expression) -> Expression {
        return Expression::Bin(BinaryExpressionKind::Equal, Box::new(lhs), Box::new(rhs));
    }

    pub fn less(lhs: Expression, rhs: Expression) -> Expression {
        return Expression::Bin(BinaryExpressionKind::Less, Box::new(lhs), Box::new(rhs));
    }

    pub fn ident(lhs: String) -> Expression {
        return Expression::Lit(Literal::Indent(lhs.to_string()));
    }

    pub fn less_equal(lhs: Expression, rhs: Expression) -> Expression {
        return Expression::Bin(
            BinaryExpressionKind::LessEqual,
            Box::new(lhs),
            Box::new(rhs),
        );
    }

    pub fn greater(lhs: Expression, rhs: Expression) -> Expression {
        return Expression::Bin(BinaryExpressionKind::Greater, Box::new(lhs), Box::new(rhs));
    }

    pub fn greater_equal(lhs: Expression, rhs: Expression) -> Expression {
        return Expression::Bin(
            BinaryExpressionKind::GreaterEqual,
            Box::new(lhs),
            Box::new(rhs),
        );
    }

    pub fn new_una(kind: UnaryExpressionKind, lhs: Expression) -> Expression {
        return Expression::Una(kind, Box::new(lhs));
    }

    pub fn not(lhs: Expression) -> Expression {
        return Expression::Una(UnaryExpressionKind::Not, Box::new(lhs));
    }

    pub fn new_lit(lit: Literal) -> Expression {
        return Expression::Lit(lit);
    }

    pub fn str(s: String) -> Expression {
        return Expression::Lit(Literal::Str(s));
    }

    pub fn int(i: i64) -> Expression {
        return Expression::Lit(Literal::Integer(i));
    }

    pub fn float(f: f64) -> Expression {
        return Expression::Lit(Literal::Float(f));
    }

    pub fn arity(&self) -> usize {
        return match self {
            Expression::Lit(_) => 0,
            Expression::Una(_, _) => 1,
            Expression::Bin(_, _, _) => 2,
        };
    }

    pub fn is_literal(&self) -> bool {
        return match self {
            Expression::Lit(_) => true,
            Expression::Una(_, _) => false,
            Expression::Bin(_, _, _) => false,
        };
    }

    pub fn is_unary(&self) -> bool {
        return match self {
            Expression::Lit(_) => false,
            Expression::Una(_, _) => true,
            Expression::Bin(_, _, _) => false,
        };
    }

    pub fn is_binary(&self) -> bool {
        return match self {
            Expression::Lit(_) => false,
            Expression::Una(_, _) => false,
            Expression::Bin(_, _, _) => true,
        };
    }

    pub fn left(&self) -> Option<&Expression> {
        return match self {
            Expression::Lit(_) => None,
            Expression::Una(_, left) => Some(left),
            Expression::Bin(_, left, _) => Some(left),
        };
    }

    pub fn right(&self) -> Option<&Expression> {
        return match self {
            Expression::Lit(_) => None,
            Expression::Una(_, _) => None,
            Expression::Bin(_, _, right) => Some(right),
        };
    }

    pub fn literal(&self) -> Option<&Literal> {
        return match self {
            Expression::Lit(lit) => Some(lit),
            Expression::Una(_, _) => None,
            Expression::Bin(_, _, _) => None,
        };
    }

    pub fn binary_kind(&self) -> Option<&BinaryExpressionKind> {
        return match self {
            Expression::Lit(_) => None,
            Expression::Una(_, _) => None,
            Expression::Bin(kind, _, _) => Some(kind),
        };
    }

    pub fn unary_kind(&self) -> Option<&UnaryExpressionKind> {
        return match self {
            Expression::Lit(_) => None,
            Expression::Una(kind, _) => Some(kind),
            Expression::Bin(_, _, _) => None,
        };
    }
}
