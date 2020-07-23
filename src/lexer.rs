mod syntax_kind;
pub(crate) use syntax_kind::SyntaxKind;

use logos::Logos;
use smol_str::SmolStr;
use std::convert::TryFrom;
use std::ops::Range;
use text_size::{TextRange, TextSize};

#[derive(Debug, PartialEq)]
pub(crate) struct Lexeme {
    pub(crate) kind: SyntaxKind,
    pub(crate) text: SmolStr,
    pub(crate) range: TextRange,
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

        let Range { start, end } = self.lexer.span();
        let start = TextSize::try_from(start).unwrap();
        let end = TextSize::try_from(end).unwrap();

        let range = TextRange::new(start, end);

        Some(Lexeme { kind, text, range })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn range(start: u32, end: u32) -> TextRange {
        TextRange::new(TextSize::from(start), TextSize::from(end))
    }

    #[test]
    fn lexer_yields_lexemes() {
        let mut lexer = Lexer::new("1 + 2");

        assert_eq!(
            lexer.next(),
            Some(Lexeme {
                kind: SyntaxKind::Number,
                text: SmolStr::from("1"),
                range: range(0, 1),
            }),
        );

        assert_eq!(
            lexer.next(),
            Some(Lexeme {
                kind: SyntaxKind::Whitespace,
                text: SmolStr::from(" "),
                range: range(1, 2),
            }),
        );

        assert_eq!(
            lexer.next(),
            Some(Lexeme {
                kind: SyntaxKind::Plus,
                text: SmolStr::from("+"),
                range: range(2, 3),
            }),
        );

        assert_eq!(
            lexer.next(),
            Some(Lexeme {
                kind: SyntaxKind::Whitespace,
                text: SmolStr::from(" "),
                range: range(3, 4),
            }),
        );

        assert_eq!(
            lexer.next(),
            Some(Lexeme {
                kind: SyntaxKind::Number,
                text: SmolStr::from("2"),
                range: range(4, 5),
            })
        );

        assert_eq!(lexer.next(), None);
    }
}
