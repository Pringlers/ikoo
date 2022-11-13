macro_rules! is {
    ($p:expr, Expr) => {{
        use $crate::token::TokenKind::*;

        is!($p, Identifier | Int | Float | Str)
    }};
    ($p:expr, $lit:literal) => {{
        let token = $p.peek();
        token.kind != $crate::token::TokenKind::Eof && &$p.code[token.span] == $lit
    }};
    ($p:expr, $pattern:pat) => {{
        let token = $p.peek();
        matches!(token.kind, $pattern)
    }};
}
pub(crate) use is;

macro_rules! eat {
    ($p:expr, $($t:tt)*) => {
        if is!($p, $($t)*) {
            $p.lexer.lex();
            true
        } else {
            false
        }
    };
}
pub(crate) use eat;

macro_rules! eat_many {
    ($p:expr, $($t:tt)*) => {
        while eat!($p, $($t)*) {}
    };
}
pub(crate) use eat_many;

macro_rules! next {
    ($p:expr) => {
        $p.lexer.lex()
    };
    ($p:expr, $($t:tt)*) => {
        if is!($p, $($t)*) {
            Some($p.lexer.lex())
        } else {
            None
        }
    };
}
pub(crate) use next;
