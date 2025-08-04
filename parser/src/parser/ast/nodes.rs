use crate::parser::ast::ast::SyntaxKind;
use crate::parser::grammar::dialect::Dialect;

///Represents the nodes of our AST
///in the simplest and generic way.
///The Dialect trait will deal with the grammar
///and compatibility between Nodes

pub enum TreeNode<D: Dialect> {
    Terminal {
        kind: SyntaxKind,
        text: String,
        _pd: PhantomData<D>,
    },
    NonTerminal {
        kind: SyntaxKind,
        children: Vec<Box<TreeNode<D>>>,
        _pd: PhantomData<D>,
    },
}

impl TreeNode<D>
where
    D: Dialect,
{
    pub fn to_tree<A>(&self) -> TreeNode<D>
    where
        A: Ast<D>,
    {
        A::from(&self)
    }

    pub(super) fn add(&mut self, child: &Self) -> Result<(), AstErr> {
        match self {
            TreeNode::Terminal => AstErr::TerminalNodeChilddAdition(node),
            TreeNode::NonTerminal => match D::are_compatible(&child, &node) {
                Ok(()) => {
                    node.children.push(&child);
                    Ok(())
                },
                Err(e) => Err(e),
            },
        }
    }
}
