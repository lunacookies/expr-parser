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

    fn test(input: &str, expected_kind: SyntaxKind) {
        let mut lexer = SyntaxKind::lexer(input);

        assert_eq!(lexer.next(), Some(expected_kind));
        assert_eq!(lexer.slice(), input);
    }

    #[test]
    fn lexes_spaces() {
        test("    ", SyntaxKind::Whitespace);
    }

    #[test]
    fn lexes_numbers() {
        test("1234567890", SyntaxKind::Number);
    }

    #[test]
    fn lexes_addition() {
        test("+", SyntaxKind::Add);
    }

    #[test]
    fn lexes_subtraction() {
        test("-", SyntaxKind::Sub);
    }

    #[test]
    fn lexes_multiplication() {
        test("*", SyntaxKind::Mul);
    }

    #[test]
    fn lexes_division() {
        test("/", SyntaxKind::Div);
    }
}
