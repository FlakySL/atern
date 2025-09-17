//use super::errors::AstErr;
use std::fmt::Display;
use super::syntax_kind::SyntaxKind;
use super::nodes::TreeNode;

pub trait Ast<K: SyntaxKind> : Display {
    fn root(&self) -> &TreeNode<K>;
    //fn diff(&self, other: &Self) -> Vec<Box<Self>>;
    fn from_node(seed: TreeNode<K>) -> Self;
    fn new() -> Self;
}
