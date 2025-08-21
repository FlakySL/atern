use crate::parser::ast::nodes::TreeNode;
use chumsky::container::Container;
use crate::parser::ast::syntax_kind::SyntaxKind;

impl Container<TreeNode> for TreeNode{
    fn push(&mut self, item: TreeNode){
        let _ = self.add(item).unwrap();
    }
}

impl Default for TreeNode{
    fn default() -> Self{
        TreeNode::NonTerminal{
            //parent: None,
            kind: SyntaxKind::ROOT,
            children: vec![],
        }
    }
}