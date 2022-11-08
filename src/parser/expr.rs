use crate::{
    ast::{BinOp, Expr, Ident, Lit, LitKind},
    token::{Token, TokenKind},
};

use super::{macros::*, Parser};

impl<'src> Parser<'src> {
    pub fn parse_expr(&mut self) -> Expr {
        // Start from highest prec
        self.parse_expr_(Token::MAX_PREC)
    }

    fn parse_expr_(&mut self, prec: u8) -> Expr {
        // Trim leading whitespace
        eat_many!(self, TokenKind::Whitespace);

        // Check if prec is lowest
        let mut lhs = if prec == Token::MIN_PREC {
            // Bind with non-binary expression
            self.parse_expr_token()
        } else {
            self.parse_expr_(prec - 1)
        };

        loop {
            // Trim whitespace before op
            eat_many!(self, TokenKind::Whitespace);

            let token = self.peek();
            if token.kind.prec() == prec {
                let op = BinOp::new(next!(self).kind).expect("cannot parse binary op");
                let rhs = self.parse_expr_(prec - 1);
                lhs = Expr::Binary {
                    op,
                    left: Box::new(lhs),
                    right: Box::new(rhs),
                };
            } else {
                return lhs;
            }
        }
    }

    fn parse_expr_token(&mut self) -> Expr {
        if !is!(self, Expr) {
            self.expected("expression");
            return Expr::Poisoned;
        }

        match self.peek().kind {
            TokenKind::Identifier => self.parse_ident_expr(),
            TokenKind::Int | TokenKind::Float | TokenKind::Str => self.parse_literal(),
            _ => unreachable!(),
        }
    }

    fn parse_ident_expr(&mut self) -> Expr {
        let Some(token) = next!(self, TokenKind::Identifier) else {
            self.expected("identifier");
            return Expr::Poisoned;
        };
        let name = Ident { span: token.span };

        if is!(self, TokenKind::LParen) {
            return match self.parse_invoke() {
                Some(args) => Expr::Call { name, args },
                None => Expr::Poisoned,
            };
        }

        Expr::Ident(name)
    }

    fn parse_literal(&mut self) -> Expr {
        use TokenKind::*;

        let Some(token) = next!(self, Int | Float | Str) else {
            self.expected("literal");
            return Expr::Poisoned;
        };

        let kind = match token.kind {
            Int => LitKind::Int,
            Float => LitKind::Float,
            Str => LitKind::Str,
            _ => unreachable!("{:?}", token.kind),
        };

        Expr::Lit(Lit {
            span: token.span,
            kind,
        })
    }

    fn parse_invoke(&mut self) -> Option<Vec<Expr>> {
        if !eat!(self, TokenKind::LParen) {
            self.expected("left parenthesis");
            return None;
        }

        let mut args = vec![];
        while !is!(self, TokenKind::RParen) {
            args.push(self.parse_expr());
            eat_many!(self, TokenKind::Whitespace);
            if !eat!(self, TokenKind::Comma) {
                break;
            }
        }

        if !eat!(self, TokenKind::RParen) {
            self.expected("right parenthesis");
            return None;
        }

        Some(args)
    }
}
