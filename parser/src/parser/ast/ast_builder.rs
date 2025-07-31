use super::ast_trait::Ast;
use super::errors::AstErr;
use super::nodes::TreeNode;
use crate::parser::grammar::dialect::Dialect;

trait AstBuilder<D, T>
where
    D: Dialect,
    T: Ast<D>,
{
    fn add(&self, node: TreeNode<D>) -> Result<(), AstErr>;
    fn tree(&self) -> &T;
    fn build(&self) -> T;
    fn checkpoint(&self) -> Option<&TreeNode<D>>;
    fn new() -> Self;
    fn back(&self) -> Option<&TreeNode<D>>;
}
