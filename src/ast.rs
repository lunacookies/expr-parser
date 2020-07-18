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
    SyntaxKind::Add,
    SyntaxKind::Mul,
    SyntaxKind::Div,
    SyntaxKind::Sub
);

impl Number {
    fn eval(&self) -> u32 {
        self.text().parse().unwrap()
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

impl Operation {
    fn eval(&self) -> Option<u32> {
        let children = self.0.children_with_tokens();

        let mut exprs = children.clone().filter_map(Expr::cast);
        let lhs = exprs.next()?.eval()?;
        let rhs = exprs.next()?.eval()?;

        let op: Op = children
            .filter_map(|element| element.into_token())
            .filter_map(Operator::cast)
            .next()?
            .into();

        let op = match op {
            Op::Add => std::ops::Add::add,
            Op::Sub => std::ops::Sub::sub,
            Op::Mul => std::ops::Mul::mul,
            Op::Div => std::ops::Div::div,
        };

        Some(op(lhs, rhs))
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

    fn eval(&self) -> Option<u32> {
        match self {
            Self::Number(n) => Some(n.eval()),
            Self::Operation(o) => o.eval(),
        }
    }
}

impl Root {
    pub(crate) fn eval(&self) -> Option<u32> {
        // Roots are expected to include only one child, with that child being an Expr.
        let expr = Expr::cast(self.0.children_with_tokens().next()?)?;
        expr.eval()
    }
}
