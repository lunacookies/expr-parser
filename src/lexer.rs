mod syntax_kind;
pub(crate) use syntax_kind::SyntaxKind;

use logos::Logos;
use smol_str::SmolStr;
use std::ops::Range;

#[derive(Debug, PartialEq)]
pub(crate) struct Lexeme {
    pub(crate) kind: SyntaxKind,
    pub(crate) text: SmolStr,
    pub(crate) range: Range<usize>,
}

pub(crate) struct Lexer<'a> {
    lexer: logos::Lexer<'a, SyntaxKind>,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(s: &'a str) -> Self {
        Self {
            lexer: SyntaxKind::lexer(s),
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Lexeme;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.lexer.next()?;
        let text = SmolStr::from(self.lexer.slice());
        let range = self.lexer.span();

        Some(Lexeme { kind, text, range })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_yields_lexemes() {
        let mut lexer = Lexer::new("1 + 2");

        assert_eq!(
            lexer.next(),
            Some(Lexeme {
                kind: SyntaxKind::Number,
                text: SmolStr::from("1"),
                range: 0..1,
            }),
        );

        assert_eq!(
            lexer.next(),
            Some(Lexeme {
                kind: SyntaxKind::Whitespace,
                text: SmolStr::from(" "),
                range: 1..2,
            }),
        );

        assert_eq!(
            lexer.next(),
            Some(Lexeme {
                kind: SyntaxKind::Plus,
                text: SmolStr::from("+"),
                range: 2..3,
            }),
        );

        assert_eq!(
            lexer.next(),
            Some(Lexeme {
                kind: SyntaxKind::Whitespace,
                text: SmolStr::from(" "),
                range: 3..4,
            }),
        );

        assert_eq!(
            lexer.next(),
            Some(Lexeme {
                kind: SyntaxKind::Number,
                text: SmolStr::from("2"),
                range: 4..5,
            })
        );

        assert_eq!(lexer.next(), None);
    }
}
