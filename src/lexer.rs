use std::{iter::Peekable, str::Chars};

use crate::{
    span::Span,
    token::{Token, TokenKind},
};

#[derive(Debug, Clone)]
pub struct Lexer<'src> {
    src: Peekable<Chars<'src>>,
    pos: u32,
}

impl<'src> Lexer<'src> {
    pub fn new(src: &'src str) -> Lexer<'src> {
        Lexer {
            src: src.chars().peekable(),
            pos: 0,
        }
    }

    pub fn lex(&mut self) -> Token {
        let Some(&current) = self.src.peek() else {
            let span = Span {
                start: self.pos,
                end: self.pos + 1,
            };
            return Token { span, kind: TokenKind::Eof };
        };

        if let Some(token) = self.parse_single(current) {
            return token;
        }

        if let Some(token) = self.parse_number() {
            return token;
        }

        if let Some(token) = self.parse_ident() {
            return token;
        }

        Token {
            span: self.capture(1),
            kind: TokenKind::Unknown,
        }
    }

    fn parse_single(&mut self, current: char) -> Option<Token> {
        let kind = match current {
            ' ' => TokenKind::Whitespace,
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '=' => TokenKind::Equal,
            '.' => TokenKind::Dot,
            ',' => TokenKind::Comma,
            _ => return None,
        };
        self.src.next();
        Some(Token {
            span: self.capture(1),
            kind,
        })
    }

    fn parse_number(&mut self) -> Option<Token> {
        let mut len = 0u32;
        while let Some(c) = self.src.peek().copied() {
            if !c.is_ascii_digit() {
                break;
            }
            len += c.len_utf8() as u32;
            self.src.next();
        }

        if len == 0 {
            return None;
        }

        // Check float
        let mut src = self.src.clone();
        if src.next() == Some('.') && matches!(src.next(), Some('0'..='9')) {
            while let Some(c) = self.src.peek().copied() {
                if !c.is_ascii_digit() {
                    break;
                }
                len += c.len_utf8() as u32;
                self.src.next();
            }

            Some(Token {
                span: self.capture(len),
                kind: TokenKind::Float,
            })
        } else {
            Some(Token {
                span: self.capture(len),
                kind: TokenKind::Int,
            })
        }
    }

    fn parse_ident(&mut self) -> Option<Token> {
        let mut len = 0u32;
        while let Some(c) = self.src.peek().copied() {
            if !c.is_ascii_alphabetic() {
                break;
            }
            len += c.len_utf8() as u32;
            self.src.next();
        }

        if len == 0 {
            None
        } else {
            Some(Token {
                span: self.capture(len),
                kind: TokenKind::Identifier,
            })
        }
    }

    fn capture(&mut self, i: u32) -> Span {
        let start = self.pos;
        self.pos += i;
        let end = self.pos;
        Span { start, end }
    }
}

pub struct CachedLexer<'src> {
    lexer: Lexer<'src>,
    cur: Option<Token>,
    next: Option<Token>,
}

impl<'src> CachedLexer<'src> {
    pub fn new(lexer: Lexer<'src>) -> CachedLexer<'src> {
        CachedLexer {
            lexer,
            cur: None,
            next: None,
        }
    }

    pub fn cur(&mut self) -> Token {
        self.cur.expect("current token is empty")
    }

    pub fn lex(&mut self) -> Token {
        self.cur = Some(match self.next {
            Some(token) => {
                self.next = None;
                token
            }
            None => self.lexer.lex(),
        });
        self.cur()
    }

    pub fn peek(&mut self) -> Token {
        match self.next {
            Some(t) => t,
            None => {
                let next = self.lex();
                self.next = Some(next);
                next
            }
        }
    }
}
