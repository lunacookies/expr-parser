mod syntax_kind;
pub(crate) use syntax_kind::SyntaxKind;

use logos::Logos;
use smol_str::SmolStr;

#[derive(Debug, PartialEq)]
pub(crate) struct Lexeme {
    pub(crate) kind: SyntaxKind,
    pub(crate) text: SmolStr,
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

        Some(Lexeme { kind, text })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_yields_smol_strs_and_syntax_kinds() {
        let mut lexer = Lexer::new("1 + 2");

        assert_eq!(
            lexer.next(),
            Some(Lexeme {
                kind: SyntaxKind::Number,
                text: SmolStr::from("1"),
            }),
        );

        assert_eq!(
            lexer.next(),
            Some(Lexeme {
                kind: SyntaxKind::Whitespace,
                text: SmolStr::from(" "),
            }),
        );

        assert_eq!(
            lexer.next(),
            Some(Lexeme {
                kind: SyntaxKind::Plus,
                text: SmolStr::from("+"),
            }),
        );

        assert_eq!(
            lexer.next(),
            Some(Lexeme {
                kind: SyntaxKind::Whitespace,
                text: SmolStr::from(" "),
            }),
        );

        assert_eq!(
            lexer.next(),
            Some(Lexeme {
                kind: SyntaxKind::Number,
                text: SmolStr::from("2"),
            })
        );

        assert_eq!(lexer.next(), None);
    }
}
