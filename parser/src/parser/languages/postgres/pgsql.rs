use crate::parser::grammar::dialect::Dialect;
use crate::parser::ast::nodes::TreeNode;

pub struct PostgreSQL{
}

impl Dialect<TreeNode> for PostgreSQL{
}