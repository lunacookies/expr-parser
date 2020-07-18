use logos::Logos;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::fmt;

#[derive(Logos, Debug, Copy, Clone, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub(crate) enum SyntaxKind {
    #[regex("[ \n]+")]
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
    Operation,
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind.into())
    }
}

impl fmt::Display for SyntaxKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Whitespace => "whitespace",
            Self::Number => "a number literal",
            Self::Add => "a plus sign",
            Self::Sub => "a minus sign",
            Self::Mul => "an asterisk",
            Self::Div => "a slash",
            Self::Error => "an erroneous character",
            _ => unreachable!(),
        })
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
    fn lexes_newlines() {
        test("\n\n", SyntaxKind::Whitespace);
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
