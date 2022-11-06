use super::{Expr, Ident};

#[derive(Debug)]
pub enum Stmt {
    Decl(Decl),
    Poisoned,
}

#[derive(Clone, Debug)]
pub struct Decl {
    pub name: Ident,
    pub value: Expr,
}
