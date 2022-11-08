use crate::{span::Span, token::TokenKind};

use super::Ident;

#[derive(Clone, Debug)]
pub enum Expr {
    Lit(Lit),
    Ident(Ident),
    Binary {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Call {
        name: Ident,
        args: Vec<Expr>,
    },
    Poisoned,
}

#[derive(Clone, Copy, Debug)]
pub struct Lit {
    pub span: Span,
    pub kind: LitKind,
}

#[derive(Clone, Copy, Debug)]
pub enum LitKind {
    Int,
    Float,
    Str,
}

#[derive(Clone, Copy, Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl BinOp {
    pub fn new(kind: TokenKind) -> Option<Self> {
        Some(match kind {
            TokenKind::Plus => Self::Add,
            TokenKind::Minus => Self::Sub,
            TokenKind::Star => Self::Mul,
            TokenKind::Slash => Self::Div,
            _ => return None,
        })
    }
}
