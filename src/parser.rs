use crate::lexer::{Lexer, SyntaxKind};
use crate::SyntaxNode;
use rowan::{GreenNode, GreenNodeBuilder};
use std::iter::Peekable;

struct Parse {
    green_node: GreenNode,
    errors: Vec<String>,
}

impl Parse {
    fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_node.clone())
    }

    fn format(&self) -> Vec<String> {
        self.syntax()
            .children_with_tokens()
            .map(|child| format!("{:?}@{:?}", child.kind(), child.text_range()))
            .collect()
    }
}

enum Op {
    Add,
    Mul,
    Div,
    Sub,
}

struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
    builder: GreenNodeBuilder<'static>,
    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    fn new(s: &'a str) -> Self {
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

    fn parse(mut self) -> Parse {
        self.builder.start_node(SyntaxKind::Root.into());

        match self.peek() {
            Some(SyntaxKind::Number) => self.bump(),
            _ => panic!("bad token"),
        }

        loop {
            let op = match self.peek() {
                None => break,
                Some(SyntaxKind::Add) => Op::Add,
                Some(SyntaxKind::Mul) => Op::Mul,
                Some(SyntaxKind::Div) => Op::Div,
                Some(SyntaxKind::Sub) => Op::Sub,
                _ => panic!("bad token"),
            };

            todo!()
        }

        self.builder.finish_node();

        Parse {
            green_node: self.builder.finish(),
            errors: self.errors,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_number() {
        let parse = Parser::new("1").parse();
        assert_eq!(parse.format(), vec!["Number@0..1"]);
    }
}
