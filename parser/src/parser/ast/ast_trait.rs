use crate::parser::ast::ast::SyntaxKind;
use crate::parser::ast::errors::AstErr;
use crate::parser::lexer::Token;
use std::fmt::Display;

pub trait AstNode: Display + Eq {
    fn add_child<A>(&mut self, child: &A) -> Result<(), AstErr>
    where
        A: AstNode;
    fn is_compatible<A>(&self, child: &A) -> Result<(), AstErr>
    where
        A: AstNode;
    fn get_parent<A>(&self) -> Option<A>
    where
        A: AstNode;
    fn get_kind<A>(&self) -> SyntaxKind;
}

pub trait AstToken: AstNode {
    fn get_token(&self) -> Token;
}

pub trait AstNodeContainer: AstNode {
    fn get_children<A>(&self) -> Vec<Box<A>>
    where
        A: AstNode;
}

pub trait Ast<N>: Iterator
where
    N: AstNode,
{
    fn diff(&self, other: &Self) -> Self;
    fn as_node(&self) -> N;
    fn from(root: &N) -> Self;
}
