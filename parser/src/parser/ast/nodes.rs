use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

use super::ast_trait::Ast;
use super::errors::AstErr;
use super::syntax_kind::SyntaxKind;



///Represents the nodes of our AST
///in the simplest and generic way.
///The Dialect trait will deal with the grammar
///and compatibility between Nodes

#[derive(Debug, Clone)]
pub enum TreeNode {
    Terminal {
        //😭
        //parent: Option<Rc<RefCell<TreeNode>>>,
        kind: SyntaxKind,
        text: String,
    },
    NonTerminal {
        //😭
        //parent: Option<Rc<RefCell<Box<TreeNode>>>>,
        kind: SyntaxKind,
        //😭
        children: Vec<Rc<RefCell<Box<TreeNode>>>>,
    },
}

impl TreeNode {
    pub fn get_kind(&self) -> SyntaxKind {
        match self {
            TreeNode::Terminal { ref kind, .. } => kind.clone(),
            TreeNode::NonTerminal { ref kind, .. } => kind.clone(),
        }
    }

    /*pub fn get_parent(&mut self) -> &mut Option<Rc<RefCell<TreeNode>>> 
    {
        match self {
            TreeNode::NonTerminal{ref mut parent,..} => parent.borrow_mut(),
            TreeNode::Terminal{ref mut parent,..} => parent.borrow_mut(),
        }
    }*/

    pub fn to_tree<A>(self) -> A
    where
        A: Ast<Self>,
    {
        A::from_node(self)
    }

    pub fn add(&mut self, child: TreeNode) -> Result<(), AstErr> {
        match self {
            TreeNode::Terminal { .. } => Err(AstErr::TerminalNodeChildAdition(self.to_string())),
            TreeNode::NonTerminal {ref mut children, .. } => {
                /*if children.is_empty() {
                    child.get_parent().borrow_mut() = Rc::new(RefCell::new(self));
                } else {
                    child.get_parent = children[0]?.get_parent().clone();
                }*/
                children.push(Rc::new(RefCell::new(Box::new(child))));
                Ok(())
            },
        }
    }
    pub fn new_term(kind: SyntaxKind, text: &str) -> TreeNode{
        TreeNode::Terminal{
            //parent: None,
            kind,
            text: text.to_string()
        }
    }
    pub fn new_term_s(kind: SyntaxKind, text: String) -> TreeNode{
        TreeNode::Terminal{
            //parent: None,
            kind,
            text
        }
    }
    pub fn new_no_term(kind: SyntaxKind, children: Vec<Box<TreeNode>> ) -> TreeNode{
        TreeNode::NonTerminal{
            //parent: None,
            kind,
            children: children.into_iter().map(|member| Rc::new(RefCell::new(member))).collect::<Vec<_>>()
        }
    }
    
    fn print(indent: usize, node: &TreeNode, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for _ in 0..indent{
            write!(f,"\t");
        };
        match node {
            Self::NonTerminal{ref kind, ref children} => {
                write!(f, "• {:?}", kind);
                for child in children {
                    writeln!(f);
                    let _ = Self::print(indent + 1, &child.borrow(), f);
                };
                std::fmt::Result::Ok(())
            },
            Self::Terminal { ref kind, ref text} => write!(f, "• {:?}: {}", kind, text)
        }
    }
}

impl Display for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = Self::print(0, self, f);
        std::fmt::Result::Ok(())
    }
}