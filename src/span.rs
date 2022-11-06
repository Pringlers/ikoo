use std::ops::Index;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Index<Span> for str {
    type Output = str;

    fn index(&self, span: Span) -> &Self::Output {
        let start = span.start as usize;
        let end = span.end as usize;
        &self[start..end]
    }
}
