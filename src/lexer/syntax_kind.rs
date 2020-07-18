use logos::Logos;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Logos, Debug, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub(crate) enum SyntaxKind {
    #[regex(" +")]
    Whitespace,

    #[regex("[1234567890]+")]
    Number,

    #[token("+")]
    Add,

    #[token("-")]
    Sub,

    #[token("*")]
    Mul,

    #[token("/")]
    Div,

    #[error]
    Error,

    Root,
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexes_nothing() {
        assert_eq!(SyntaxKind::lexer("").next(), None);
    }

    #[test]
    fn lexes_spaces() {
        let mut lexer = SyntaxKind::lexer("    ");

        assert_eq!(lexer.next(), Some(SyntaxKind::Whitespace));
        assert_eq!(lexer.slice(), "    ");
    }

    #[test]
    fn lexes_numbers() {
        let mut lexer = SyntaxKind::lexer("1234567890");

        assert_eq!(lexer.next(), Some(SyntaxKind::Number));
        assert_eq!(lexer.slice(), "1234567890");
    }

    #[test]
    fn lexes_addition() {
        let mut lexer = SyntaxKind::lexer("+");

        assert_eq!(lexer.next(), Some(SyntaxKind::Add));
        assert_eq!(lexer.slice(), "+");
    }

    #[test]
    fn lexes_subtraction() {
        let mut lexer = SyntaxKind::lexer("-");

        assert_eq!(lexer.next(), Some(SyntaxKind::Sub));
        assert_eq!(lexer.slice(), "-");
    }

    #[test]
    fn lexes_multiplication() {
        let mut lexer = SyntaxKind::lexer("*");

        assert_eq!(lexer.next(), Some(SyntaxKind::Mul));
        assert_eq!(lexer.slice(), "*");
    }

    #[test]
    fn lexes_division() {
        let mut lexer = SyntaxKind::lexer("/");

        assert_eq!(lexer.next(), Some(SyntaxKind::Div));
        assert_eq!(lexer.slice(), "/");
    }
}
