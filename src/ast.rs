use crate::span::Span;

pub use {expr::*, stmt::*};

mod expr;
mod stmt;

#[derive(Debug)]
pub struct Module {
    pub stmts: Vec<Stmt>,
}

#[derive(Clone, Copy, Debug)]
pub struct Ident {
    pub span: Span,
}
