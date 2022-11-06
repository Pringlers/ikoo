use crate::{
    ast::Module,
    lexer::{CachedLexer, Lexer},
    span::Span,
    token::{Token, TokenKind},
};

use self::macros::is;

mod expr;
mod macros;
mod stmt;

pub struct Parser<'src> {
    code: &'src str,
    lexer: CachedLexer<'src>,
    errors: Vec<ParseError>,
}

impl<'src> Parser<'src> {
    pub fn new(code: &'src str, lexer: Lexer<'src>) -> Parser<'src> {
        Parser {
            code,
            lexer: CachedLexer::new(lexer),
            errors: Vec::new(),
        }
    }

    pub fn parse_module(&mut self) -> Module {
        let mut module = Module { stmts: vec![] };
        loop {
            if is!(self, "let") {
                module.stmts.push(self.parse_let());
            } else {
                return module;
            }
        }
    }

    fn report_error(&mut self, err: ParseError) {
        self.errors.push(err);
    }

    fn expected(&mut self, expected: &'static str) {
        let token = self.cur();
        self.report_error(ParseError {
            span: token.span,
            kind: ParseErrorKind::Unexpected {
                expected,
                found: token.kind,
            },
        });
    }

    fn peek(&mut self) -> Token {
        self.lexer.peek()
    }

    fn cur(&mut self) -> Token {
        self.lexer.cur()
    }
}

#[derive(Debug)]
pub struct ParseError {
    pub span: Span,
    pub kind: ParseErrorKind,
}

#[derive(Debug)]
pub enum ParseErrorKind {
    Unexpected {
        expected: &'static str,
        found: TokenKind,
    },
}
