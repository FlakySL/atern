<<<<<<< HEAD
use super::errors::AstErr;
use super::nodes::TreeNode;
use crate::parser::grammar::dialect::Dialect;
=======
<<<<<<< HEAD
use crate::parser::ast::ast::SyntaxKind;
use crate::parser::ast::errors::AstErr;
use crate::parser::lexer::Token;
use std::fmt::Display;
>>>>>>> a22607f (chore!: rowan removed from the project. Parser implementation needed.)

pub trait Ast<D>
where
    D: Dialect,
{
<<<<<<< HEAD
=======
    fn diff(&self, other: &Self) -> Self;
    fn as_node(&self) -> N;
    fn from(root: &N) -> Self;
=======
use super::errors::AstErr;
use super::nodes::TreeNode;
use crate::parser::grammar::dialect::Dialect;

pub trait Ast<D>
where
    D: Dialect,
{
>>>>>>> a22607f (chore!: rowan removed from the project. Parser implementation needed.)
    fn root(&self) -> Option<Box<TreeNode<D>>>;
    fn take_root(&mut self, node: TreeNode<D>) -> Result<(), AstErr>;
    fn diff(&self, other: &Self) -> Vec<Box<Self>>;
    fn from_node(seed: TreeNode<D>) -> Self;
    fn new() -> Self;
<<<<<<< HEAD
=======
>>>>>>> 6613d25 (chore!: rowan removed from the project. Parser implementation needed.)
>>>>>>> a22607f (chore!: rowan removed from the project. Parser implementation needed.)
}
