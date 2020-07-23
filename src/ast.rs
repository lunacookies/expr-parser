mod eval;

use crate::lexer::SyntaxKind;
use crate::{Op, SyntaxElement, SyntaxNode, SyntaxToken};
use rowan::SmolStr;

macro_rules! ast_node {
    ($name:ident, $($syntax_kind:expr),+) => {
        #[allow(unused)]
        #[derive(Debug)]
        pub(crate) struct $name(SyntaxNode);

        impl $name {
            #[allow(unused)]
            pub(crate) fn cast(node: SyntaxNode) -> Option<Self> {
                if $(node.kind() == $syntax_kind)||+ {
                    Some(Self(node))
                } else {
                    None
                }
            }

            #[allow(unused)]
            fn text(&self) -> &SmolStr {
                match &self.0.green().children().next() {
                    Some(rowan::NodeOrToken::Token(token)) => token.text(),
                    _ => unreachable!(),
                }
            }
        }
    };
}

macro_rules! ast_token {
    ($name:ident, $($syntax_kind:expr),+) => {
        #[allow(unused)]
        #[derive(Debug)]
        pub(crate) struct $name(SyntaxToken);

        impl $name {
            #[allow(unused)]
            pub(crate) fn cast(node: SyntaxToken) -> Option<Self> {
                if $(node.kind() == $syntax_kind)||+ {
                    Some(Self(node))
                } else {
                    None
                }
            }

            #[allow(unused)]
            fn text(&self) -> &SmolStr {
                self.0.text()
            }
        }
    };
}

ast_node!(Root, SyntaxKind::Root);
ast_node!(Operation, SyntaxKind::Operation);

ast_token!(Number, SyntaxKind::Number);
ast_token!(
    Operator,
    SyntaxKind::Plus,
    SyntaxKind::Star,
    SyntaxKind::Slash,
    SyntaxKind::Minus
);

impl Operation {
    fn lhs(&self) -> Option<Expr> {
        self.0.children_with_tokens().filter_map(Expr::cast).next()
    }

    fn op(&self) -> Option<Operator> {
        self.0
            .children_with_tokens()
            .filter_map(|element| element.into_token())
            .filter_map(Operator::cast)
            .next()
    }

    fn rhs(&self) -> Option<Expr> {
        self.0.children_with_tokens().filter_map(Expr::cast).nth(1)
    }
}

impl From<Operator> for Op {
    fn from(op: Operator) -> Self {
        match op.text().as_str() {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Expr {
    Number(Number),
    Operation(Operation),
}

impl Expr {
    fn cast(element: SyntaxElement) -> Option<Self> {
        element
            .clone()
            .into_token()
            .and_then(Number::cast)
            .map(Self::Number)
            .or_else(|| {
                element
                    .clone()
                    .into_node()
                    .and_then(Operation::cast)
                    .map(Self::Operation)
            })
    }
}
