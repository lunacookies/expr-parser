use crate::lexer::SyntaxKind;
use crate::{Op, SyntaxNode};
use rowan::SmolStr;

macro_rules! ast_node {
    ($name:ident, $($syntax_kind:expr),+) => {
        struct $name(SyntaxNode);

        impl $name {
            #[allow(unused)]
            fn cast(node: SyntaxNode) -> Option<Self> {
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

ast_node!(Root, SyntaxKind::Root);
ast_node!(Number, SyntaxKind::Number);
ast_node!(
    Operator,
    SyntaxKind::Add,
    SyntaxKind::Mul,
    SyntaxKind::Div,
    SyntaxKind::Sub
);
ast_node!(Operation, SyntaxKind::Operation);

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
        let children = self.0.children();

        let mut exprs = children.clone().filter_map(Expr::cast);
        let lhs = exprs.next()?.eval()?;
        let rhs = exprs.next()?.eval()?;

        let op: Op = children.filter_map(Operator::cast).next()?.into();

        let op = match op {
            Op::Add => std::ops::Add::add,
            Op::Sub => std::ops::Sub::sub,
            Op::Mul => std::ops::Mul::mul,
            Op::Div => std::ops::Div::div,
        };

        Some(op(lhs, rhs))
    }
}

enum Expr {
    Number(Number),
    Operation(Operation),
}

impl Expr {
    fn cast(node: SyntaxNode) -> Option<Self> {
        Number::cast(node.clone())
            .map(Self::Number)
            .or_else(|| Operation::cast(node.clone()).map(Self::Operation))
    }

    fn eval(&self) -> Option<u32> {
        match self {
            Self::Number(n) => Some(n.eval()),
            Self::Operation(o) => o.eval(),
        }
    }
}

impl Root {
    fn eval(&self) -> Option<u32> {
        // Roots are expected to include only one child, with that child being an Expr.
        let expr = Expr::cast(self.0.children().next()?)?;
        expr.eval()
    }
}
