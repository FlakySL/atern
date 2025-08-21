use super::ast_trait::Ast;
use super::errors::AstErr;
use super::nodes::TreeNode;

trait AstBuilder<D, N, T>
where
    T: Ast<N>,
{
    fn add(&self, node: N) -> Result<(), AstErr>;
    fn tree(&self) -> &T;
    fn build(&self) -> T;
    fn checkpoint(&self) -> Option<&TreeNode>;
    fn new() -> Self;
    fn back(&self) -> Option<&TreeNode>;
}
