use super::errors::AstErr;
use super::nodes::TreeNode;
use crate::parser::grammar::dialect::Dialect;

pub trait Ast<D>
where
    D: Dialect,
{
    fn root(&self) -> Option<Box<TreeNode<D>>>;
    fn take_root(&mut self, node: TreeNode<D>) -> Result<(), AstErr>;
    fn diff(&self, other: &Self) -> Vec<Box<Self>>;
    fn from_node(seed: TreeNode<D>) -> Self;
    fn new() -> Self;
}
