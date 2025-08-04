use super::nodes::TreeNode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AstErr {
    #[error("Terminal node: {0} cannot have children")]
    TerminalNodeChildAdition(TreeNode::Terminal),
}
