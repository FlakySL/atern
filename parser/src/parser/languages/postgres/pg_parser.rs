use chumsky::prelude::*;
use chumsky::input::{Stream,ValueInput};
use chumsky::error::Rich;
use chumsky::extra::Err as CE;
use chumsky::Parser as CP;
use crate::parser::parser::parser::Parser as PP;
use crate::parser::lexer::Token;
use logos::Logos;
use super::pgsql::PostgreSQL;
use crate::parser::ast::syntax_kind::SyntaxKind;
use crate::parser::ast::nodes::TreeNode;
use chumsky::container::Container;

//pub(super) type MiniParser = impl CP<'src, TokenInput<'src>, TreeNode, CE<'src, Rich<'src, PgAst, Token>>>;

pub struct PgParser<'src>{
  source: &'src str,
}

impl <'src> PP<'src, TreeNode, PostgreSQL, Token> for PgParser<'src>{
    fn new(source: &'src str) -> Self{
        PgParser{
            source
        }
    }
    
    fn parse(&mut self) -> ParseResult<TreeNode, Rich<'src,Token>>{
        let token_iter = Token::lexer(self.source)
                        .spanned()
                        .map(|(tok, span)| match tok {
                            Ok(tok) => (tok, span.into()),
                            Err(()) => (Token::Error, span.into()),
                        });

    let token_stream = Stream::from_iter(token_iter)
                        .map((0..self.source.len()).into(), |(t, s)| (t, s));

        start().parse(token_stream)
    }
}
//=====================PARSERS=================//
fn start<'src, I>() -> impl CP<'src, I ,TreeNode, CE<Rich<'src,Token>>>
where
    I: ValueInput<'src, Token = Token, Span = SimpleSpan>
{
    create_table()
}

fn create_table<'src,I>() -> impl CP<'src, I ,TreeNode, CE<Rich<'src, Token>>>
where
    I: ValueInput<'src, Token = Token, Span = SimpleSpan>
{
    let create = just(Token::Create).to(TreeNode::new_no_term(SyntaxKind::CREATE,vec![]));
    let table = just(Token::Table).to(TreeNode::new_no_term(SyntaxKind::TABLE, vec![]));
    let table_name = select!{
                Token::Identifier(x) => TreeNode::new_term(SyntaxKind::NAME, x),
    };
    create
        .then(table)
        .then(table_name)
        .then(
            column()
                .map(Box::new)
                .separated_by(just(Token::Comma))
                .collect::<Vec<_>>()
                .delimited_by(
                    just(Token::ParenthesesStart),
                    just(Token::ParenthesesEnd)
                        .ignore_then(just(Token::Semicolon))
                )
                
        )
        .map(|(((mut c,mut t),tn),cols)| {
            t.push(tn);
            t.push(TreeNode::new_no_term(SyntaxKind::TABLE_BODY, cols));
            c.push(t);
            c
        })
}

fn column<'src,I>() -> impl CP<'src, I ,TreeNode, CE<Rich<'src, Token>>>
where
    I: ValueInput<'src, Token = Token, Span = SimpleSpan>
{
    let column_name = select!{
                Token::Identifier(x) => TreeNode::new_term(SyntaxKind::NAME, x),
    };
    let column_type = select!{Token::Identifier(type_name) => TreeNode::new_term(SyntaxKind::TYPE, type_name),
    };
    let column_constraints = choice((
        just(Token::Null).to(TreeNode::new_term(SyntaxKind::NULL, "NULL".to_string())),
        just(Token::Not).then(just(Token::Null)).to(TreeNode::new_term(SyntaxKind::NOT_NULL, "NOT NULL".to_string())),
        just(Token::Primary).then(just(Token::Key)).to(TreeNode::new_term(SyntaxKind::PRIMARY_KEY, "PRYMARY KEY".to_string())),
        just(Token::Unique).then(just(Token::Nulls).then(just(Token::Not).or_not()).then(just(Token::Distinct)).or_not()).map(|(_, n)| {
            let mut node = TreeNode::new_no_term(SyntaxKind::UNIQUE, vec![]);
            if let Some(((_,no), di)) = n {
                node.push(TreeNode::new_term(SyntaxKind::NULLS, "NULLS".to_string()));
                if let (Some(Token::Not), Token::Distinct) = (no, di) {
                    node.push(TreeNode::new_term(SyntaxKind::NOT, "NOT".to_string()));
                    node.push(TreeNode::new_term(SyntaxKind::DISTINCT, "DISTINCT".to_string()));
                }
                else {
                    node.push(TreeNode::new_term(SyntaxKind::DISTINCT, "DISTINCT".to_string()));
                }
            }
            node
        }),
        just(Token::References).ignore_then(select!{ Token::Identifier(x) => TreeNode::new_term(SyntaxKind::NAME, x) }).map(|tn|
            TreeNode::new_no_term(SyntaxKind::FOREGEIN_KEY, vec![Box::new(tn)])
            )
    ))
    .map(Box::new)
    .repeated()
    .collect::<Vec<_>>()
    .map(|v| TreeNode::new_no_term(SyntaxKind::COLUMN_CONSTRAINTS, v));
    
    column_name
        .then(column_type)
        .then(column_constraints)
        .map(|((n,t), c)| TreeNode::new_no_term(SyntaxKind::COLUMN, vec![Box::new(n), Box::new(t), Box::new(c)]))
}