use std::cell::RefCell;
use std::fmt::Display;
use std::marker::PhantomData;
use std::rc::Rc;
use std::rc::Weak;

use super::ast_trait::Ast;
use super::errors::AstErr;
use super::syntax_kind::SyntaxKind;
use crate::parser::grammar::dialect::Dialect;

///Represents the nodes of our AST
///in the simplest and generic way.
///The Dialect trait will deal with the grammar
///and compatibility between Nodes

#[derive(Debug)]
pub enum TreeNode<D: Dialect> {
    Terminal {
        //😭
        parent: RefCell<Option<Weak<TreeNode<D>>>>,
        kind: SyntaxKind,
        text: String,
        _pd: PhantomData<D>,
    },
    NonTerminal {
        //😭
        parent: RefCell<Option<Weak<TreeNode<D>>>>,
        kind: SyntaxKind,
        //😭
        children: Vec<RefCell<Rc<TreeNode<D>>>>,
        _pd: PhantomData<D>,
    },
}

impl<D: Dialect> TreeNode<D> {
    pub fn get_kind(&self) -> SyntaxKind {
        match self {
            TreeNode::Terminal { ref kind, .. } => kind.clone(),
            TreeNode::NonTerminal { ref kind, .. } => kind.clone(),
        }
    }

    pub fn to_tree<A>(self) -> A
    where
        A: Ast<D>,
    {
        A::from_node(self)
    }

    pub(super) fn add(&mut self, child: Self) -> Result<(), AstErr> {
        D::are_compatible(&child, self)?;
        match self {
            TreeNode::Terminal { .. } => Err(AstErr::TerminalNodeChildAdition(self.to_string())),
            TreeNode::NonTerminal { ref mut children, .. } => {
                children.push(RefCell::new(Rc::new(child)));
                Ok(())
            },
        }
    }
}

impl<D: Dialect> Display for TreeNode<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Terminal { ref kind, ref text, .. } => write!(f, "{:?} - {}", kind, text),
            Self::NonTerminal { kind, children, .. } => {
                write!(
                    f,
                    "{:?} - {:?} ",
                    kind,
                    children
                        .iter()
                        .map(|child| (**child.borrow()).get_kind())
                        .collect::<Vec<SyntaxKind>>()
                )
            },
        }
    }
}
