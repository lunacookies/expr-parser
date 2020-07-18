mod ast;
mod lang;
mod lexer;

use lexer::SyntaxKind;

type SyntaxNode = rowan::SyntaxNode<lang::Lang>;
