use super::ast_trait::Ast;
use super::errors::AstErr;
use super::nodes::TreeNode;
use crate::parser::grammar::dialect::Dialect;

pub struct AstBuilder<'a, T>
where
    D: Dialect,
    T: Ast<D>,
{
    tree: T,
    pointed: Option<&'a TreeNode<D>>,
}

impl AstBuilder<'_, T>
where
    D: Dialect,
    T: Ast<D>,
{
    pub fn new() -> Self {
        Self { tree: D::tree_seed(), None }
    }

    pub fn add(node: &TreeNode<D>) -> Result<(), AstErr>
    where
        N: AstNode,
    {
        tree.add(node)
    }
}
