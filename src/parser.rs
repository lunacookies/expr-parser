use crate::ast::Root;
use crate::lexer::{Lexer, SyntaxKind};
use crate::{Op, SyntaxNode};
use rowan::{GreenNode, GreenNodeBuilder};
use std::iter::Peekable;

pub struct Parse {
    green_node: GreenNode,
    errors: Vec<String>,
}

impl Parse {
    fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_node.clone())
    }

    pub fn eval(&self) -> Option<u32> {
        // Parse will always contain a Root node, so we can unwrap.
        Root::cast(self.syntax()).unwrap().eval()
    }

    pub fn errors(&self) -> &[String] {
        &self.errors
    }

    pub fn format(&self) -> String {
        format!("{:#?}", self.syntax())
    }
}

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
    builder: GreenNodeBuilder<'static>,
    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            lexer: Lexer::new(s).peekable(),
            builder: GreenNodeBuilder::new(),
            errors: Vec::new(),
        }
    }

    fn peek(&mut self) -> Option<SyntaxKind> {
        self.lexer.peek().map(|(kind, _)| *kind)
    }

    fn bump(&mut self) {
        let (kind, text) = self.lexer.next().unwrap();
        self.builder.token(kind.into(), text);
    }

    fn eat(&mut self, kind: SyntaxKind) {
        let (_, text) = self.lexer.next().unwrap();
        self.builder.token(kind.into(), text);
    }

    fn skip_ws(&mut self) {
        while self.peek() == Some(SyntaxKind::Whitespace) {
            self.bump();
        }
    }

    pub fn parse(mut self) -> Parse {
        self.builder.start_node(SyntaxKind::Root.into());

        self.skip_ws();
        self.expr_bp(0);
        self.skip_ws();

        self.builder.finish_node();

        Parse {
            green_node: self.builder.finish(),
            errors: self.errors,
        }
    }

    fn expr_bp(&mut self, min_bp: u8) {
        let checkpoint = self.builder.checkpoint();

        loop {
            match self.peek() {
                Some(SyntaxKind::Number) => {
                    self.bump();
                    break;
                }
                Some(kind) => {
                    self.eat(SyntaxKind::Error);
                    self.errors
                        .push(format!("found {}, expected {}", kind, SyntaxKind::Number));
                }
                None => {
                    self.errors.push(format!("expected {}", SyntaxKind::Number));
                    return;
                }
            }
        }

        self.skip_ws();

        loop {
            let op = loop {
                match self.peek() {
                    Some(SyntaxKind::Plus) => {
                        break Op::Add;
                    }
                    Some(SyntaxKind::Star) => {
                        break Op::Mul;
                    }
                    Some(SyntaxKind::Slash) => {
                        break Op::Div;
                    }
                    Some(SyntaxKind::Minus) => {
                        break Op::Sub;
                    }
                    Some(kind) => {
                        self.eat(SyntaxKind::Error);
                        self.errors
                            .push(format!("found {}, expected an operator", kind));
                    }
                    None => return,
                }
            };

            let (left_bp, right_bp) = infix_bp(op);

            if left_bp < min_bp {
                break;
            }

            // Only continue building the syntax tree after potentially breaking out of the loop to
            // prevent a half-built syntax tree.

            self.builder
                .start_node_at(checkpoint, SyntaxKind::Operation.into());

            // Eat the operator’s token and any whitespace following it.
            self.bump();
            self.skip_ws();

            self.expr_bp(right_bp);

            self.builder.finish_node();
        }
    }
}

fn infix_bp(op: Op) -> (u8, u8) {
    match op {
        Op::Add | Op::Sub => (1, 2),
        Op::Mul | Op::Div => (3, 4),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_single_number() {
        let parse = Parser::new("1").parse();

        assert_eq!(
            parse.format(),
            r#"Root@0..1
  Number@0..1 "1"
"#,
        );
    }

    #[test]
    fn parse_simple_binary_operation() {
        let parse = Parser::new("1+1").parse();

        assert_eq!(
            parse.format(),
            r#"Root@0..3
  Operation@0..3
    Number@0..1 "1"
    Plus@1..2 "+"
    Number@2..3 "1"
"#,
        );
    }

    #[test]
    fn multiplication_has_higher_precedence_than_addition() {
        let parse = Parser::new("1+2*3").parse();

        assert_eq!(
            parse.format(),
            r#"Root@0..5
  Operation@0..5
    Number@0..1 "1"
    Plus@1..2 "+"
    Operation@2..5
      Number@2..3 "2"
      Star@3..4 "*"
      Number@4..5 "3"
"#,
        );
    }

    #[test]
    fn subtraction_is_left_associative() {
        let parse = Parser::new("10-7-3").parse();

        assert_eq!(
            parse.format(),
            r#"Root@0..6
  Operation@0..6
    Operation@0..4
      Number@0..2 "10"
      Minus@2..3 "-"
      Number@3..4 "7"
    Minus@4..5 "-"
    Number@5..6 "3"
"#,
        );
    }

    #[test]
    fn whitespace_is_skipped() {
        let parse = Parser::new(" 14 +26- 27 /  3 * 2 ").parse();

        assert_eq!(
            parse.format(),
            r#"Root@0..21
  Whitespace@0..1 " "
  Operation@1..21
    Operation@1..7
      Number@1..3 "14"
      Whitespace@3..4 " "
      Plus@4..5 "+"
      Number@5..7 "26"
    Minus@7..8 "-"
    Whitespace@8..9 " "
    Operation@9..21
      Operation@9..17
        Number@9..11 "27"
        Whitespace@11..12 " "
        Slash@12..13 "/"
        Whitespace@13..15 "  "
        Number@15..16 "3"
        Whitespace@16..17 " "
      Star@17..18 "*"
      Whitespace@18..19 " "
      Number@19..20 "2"
      Whitespace@20..21 " "
"#,
        );
    }

    #[test]
    fn junk_before_numbers_is_skipped() {
        let parse = Parser::new("abc1").parse();

        assert_eq!(
            parse.format(),
            r#"Root@0..4
  Error@0..1 "a"
  Error@1..2 "b"
  Error@2..3 "c"
  Number@3..4 "1"
"#,
        );
    }

    #[test]
    fn junk_before_operators_is_skipped() {
        let parse = Parser::new("1 a+ 2").parse();

        assert_eq!(
            parse.format(),
            r#"Root@0..6
  Operation@0..6
    Number@0..1 "1"
    Whitespace@1..2 " "
    Error@2..3 "a"
    Plus@3..4 "+"
    Whitespace@4..5 " "
    Number@5..6 "2"
"#,
        )
    }
}
