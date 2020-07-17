use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
enum SyntaxKind {
    #[regex(" +")]
    Whitespace,

    #[regex("[1234567890]+")]
    Number,

    #[regex("[*/+-]")]
    Op,

    #[error]
    Error,
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
    fn lexes_operations() {
        let mut lexer = SyntaxKind::lexer("+-*/");

        assert_eq!(lexer.next(), Some(SyntaxKind::Op));
        assert_eq!(lexer.slice(), "+");
        assert_eq!(lexer.next(), Some(SyntaxKind::Op));
        assert_eq!(lexer.slice(), "-");
        assert_eq!(lexer.next(), Some(SyntaxKind::Op));
        assert_eq!(lexer.slice(), "*");
        assert_eq!(lexer.next(), Some(SyntaxKind::Op));
        assert_eq!(lexer.slice(), "/");
    }
}
