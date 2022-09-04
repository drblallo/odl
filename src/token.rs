#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Ident(String),

    Integer(i64),
    Equals,
    Different,

    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    Plus,
    Minus,
    Or,
    And,
    Star,
    Slash,
    LParen,
    RParen,
    Semi,

    Whitespace(i64),
    Comment,

    EndLine,
    Indent,
    Deindent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub lo: usize,
    pub hi: usize,
}

impl Span {
    pub fn merge(self, other: &Span) -> Span {
        let lo = self.lo;
        let hi = other.hi;
        return Span { lo, hi };
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, lo: usize, hi: usize) -> Token {
        Token {
            kind,
            span: Span { lo, hi },
        }
    }

    pub fn is_whitespace(&self) -> bool {
        match self.kind {
            TokenKind::Whitespace(_a) => return true,
            _ => return false,
        };
    }

    pub fn try_merge_whitespace(&self, other: &Token) -> Option<Token> {
        if let (
            Token {
                kind: TokenKind::Whitespace(a),
                span: s1,
            },
            Token {
                kind: TokenKind::Whitespace(b),
                span: s2,
            },
        ) = (self, other)
        {
            return Some(Token {
                kind: TokenKind::Whitespace(a + b),
                span: s1.merge(&s2),
            });
        }
        return None;
    }

    pub fn get_identifier(&self) -> Option<String> {
        match self.kind.clone() {
            TokenKind::Ident(s) => Some(s),
            _ => None,
        }
    }

    pub fn get_float(&self) -> Option<i64> {
        match self.kind {
            _ => None,
        }
    }

    pub fn get_int(&self) -> Option<i64> {
        match self.kind {
            TokenKind::Integer(a) => return Some(a),
            _ => None,
        }
    }
}
