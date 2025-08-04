use super::nodes::TreeNode;
use crate::parser::grammar::dialect::Dialect;

pub trait Ast<D>
where
    D: Dialect,
{
    fn root(&self) -> &TreeNode<D>;
    fn diff(&self, other: &Self) -> Vec<Box<Self>>;
    fn from_node(seed: &TreeNode<D>) -> Self;
}
