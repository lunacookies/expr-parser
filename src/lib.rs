mod ast;
mod lang;
mod lexer;
mod parser;

type SyntaxNode = rowan::SyntaxNode<lang::Lang>;
