use crate::lexer::SyntaxKind;
use crate::SyntaxNode;

macro_rules! ast_node {
    ($name: ident, $syntax_kind: expr) => {
        struct $name(SyntaxNode);

        impl $name {
            fn cast(node: SyntaxNode) -> Option<Self> {
                if node.kind() == $syntax_kind {
                    Some(Self(node))
                } else {
                    None
                }
            }
        }
    };
}

ast_node!(Root, SyntaxKind::Root);
ast_node!(Number, SyntaxKind::Number);
ast_node!(Add, SyntaxKind::Add);
ast_node!(Mul, SyntaxKind::Mul);
ast_node!(Div, SyntaxKind::Div);
ast_node!(Sub, SyntaxKind::Sub);
ast_node!(Operation, SyntaxKind::Operation);

struct Expr(SyntaxNode);

enum ExprKind {
    Number(Number),
    Operation(Operation),
}

impl Expr {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if Number::cast(node.clone()).is_some() || Operation::cast(node.clone()).is_some() {
            Some(Self(node))
        } else {
            None
        }
    }

    fn kind(&self) -> ExprKind {
        Number::cast(self.0.clone())
            .map(ExprKind::Number)
            .or_else(|| Operation::cast(self.0.clone()).map(ExprKind::Operation))
            .unwrap()
    }
}
