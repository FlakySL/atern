use crate::parser::ast::ast_trait::Ast;
use crate::parser::ast::nodes::TreeNode;
use super::pg_kind::PgKind;
use std::borrow::Borrow;
use std::fmt::Display;

pub struct PgAst{
    root: TreeNode<PgKind>
}

impl Ast<PgKind> for PgAst {
    fn new() -> Self{
        PgAst{
            root: TreeNode::default()
        }
    }
    fn root(&self) -> &TreeNode<PgKind>{
        self.root.borrow()
    }
    fn from_node(seed: TreeNode<PgKind>) -> Self{
        let mut instance = Self::new();
        instance.root.add(seed).unwrap();
        instance
    }
}

impl Display for PgAst{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        self.root.fmt(f)
    }
}