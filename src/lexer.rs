mod syntax_kind;
pub(crate) use syntax_kind::SyntaxKind;

use logos::Logos;
use smol_str::SmolStr;

struct Lexer<'a> {
    lexer: logos::Lexer<'a, SyntaxKind>,
}

impl<'a> Lexer<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            lexer: SyntaxKind::lexer(s),
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = (SyntaxKind, SmolStr);

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.lexer.next()?;
        let text = SmolStr::from(self.lexer.slice());

        Some((kind, text))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_yields_smol_strs_and_syntax_kinds() {
        let mut lexer = Lexer::new("1 + 2");

        assert_eq!(lexer.next(), Some((SyntaxKind::Number, SmolStr::from("1"))));

        assert_eq!(
            lexer.next(),
            Some((SyntaxKind::Whitespace, SmolStr::from(" ")))
        );

        assert_eq!(lexer.next(), Some((SyntaxKind::Op, SmolStr::from("+"))));

        assert_eq!(
            lexer.next(),
            Some((SyntaxKind::Whitespace, SmolStr::from(" ")))
        );

        assert_eq!(lexer.next(), Some((SyntaxKind::Number, SmolStr::from("2"))));

        assert_eq!(lexer.next(), None);
    }
}
