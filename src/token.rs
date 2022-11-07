use crate::span::Span;

#[derive(Debug, Copy, Clone)]
pub struct Token {
    pub span: Span,
    pub kind: TokenKind,
}

impl Token {
    pub const MIN_PREC: u8 = 0;
    pub const MAX_PREC: u8 = 2;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Eof,
    Unknown,
    Whitespace,
    Identifier,
    Str,
    Int,
    Float,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    Dot,
    Comma,
}

impl TokenKind {
    pub fn prec(self) -> u8 {
        use TokenKind::*;

        match self {
            Star | Slash => 1,
            Plus | Minus => Token::MAX_PREC,
            _ => u8::MAX,
        }
    }
}
