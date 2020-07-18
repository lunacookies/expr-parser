mod ast;
mod lang;
mod lexer;
mod parser;

type SyntaxNode = rowan::SyntaxNode<lang::Lang>;
type SyntaxToken = rowan::SyntaxToken<lang::Lang>;
type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;

enum Op {
    Add,
    Mul,
    Div,
    Sub,
}

pub use parser::Parser;
