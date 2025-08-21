use crate::parser::ast::ast_trait::Ast;
use crate::parser::ast::nodes::TreeNode;
use std::borrow::Borrow;
use std::fmt::Display;

pub struct PgAst{
    root: TreeNode
}

impl Ast<TreeNode> for PgAst {
    fn new() -> Self{
        PgAst{
            root: TreeNode::default()
        }
    }
    fn root(&self) -> &TreeNode{
        self.root.borrow()
    }
    fn from_node(seed: TreeNode) -> Self{
        let mut instance = Self::new();
        let _ = instance.root.add(seed).unwrap();
        instance
    }
}

impl Display for PgAst{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        self.root.fmt(f)
    }
}