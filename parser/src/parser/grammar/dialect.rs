use crate::parser::ast::errors::AstErr;
use crate::parser::ast::nodes::TreeNode;

///This trait deals with the grammar of the SQL dialect we want to parse.
///We proly need to add more methods in the future.
pub trait Dialect {
    fn are_compatible(child: &TreeNode<Self>, parent: &TreeNode<Self>) -> Result<(), AstErr>;
}
