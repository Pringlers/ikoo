use crate::{
    ast::{Decl, Ident, Stmt},
    parser::macros::*,
    token::TokenKind,
};

use super::Parser;

impl<'src> Parser<'src> {
    pub fn parse_let(&mut self) -> Stmt {
        use TokenKind::*;

        assert!(eat!(self, "let"));

        if !eat!(self, Whitespace) {
            self.expected("whitespace");
            return Stmt::Poisoned;
        }
        eat_many!(self, Whitespace);

        let name = match next!(self, Identifier) {
            Some(t) => t,
            None => {
                self.expected("identifier");
                return Stmt::Poisoned;
            }
        };

        eat_many!(self, Whitespace);
        if !eat!(self, Equal) {
            self.expected("whitespace");
            return Stmt::Poisoned;
        }
        eat_many!(self, Whitespace);

        let expr = self.parse_expr();

        Stmt::Decl(Decl {
            name: Ident { span: name.span },
            value: expr,
        })
    }
}
