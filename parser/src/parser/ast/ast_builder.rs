use super::ast_trait::Ast;
use super::errors::AstErr;
use super::nodes::TreeNode;
use super::syntax_kind::SyntaxKind;

trait AstBuilder<D, K, T>
where
    K: SyntaxKind,
    T: Ast<K>,
{
    fn add(&self, node: TreeNode<K>) -> Result<(), AstErr>;
    fn tree(&self) -> &T;
    fn build(&self) -> T;
    fn checkpoint(&self) -> Option<&TreeNode<K>>;
    fn new() -> Self;
    fn back(&self) -> Option<&TreeNode<K>>;
}
