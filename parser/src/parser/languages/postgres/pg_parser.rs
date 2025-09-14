use std::{
    collections::LinkedList,
    cell::RefCell,
    rc::Rc,
    boxed::Box,
};

use crate::parser::parser::parser::Parser as PP;
use crate::parser::lexer::Token;
use logos::Logos;
use super::pgsql::PostgreSQL;
use crate::boxed_vec;
use crate::parser::ast::syntax_kind::SyntaxKind;
use crate::parser::ast::nodes::TreeNode;
use chumsky::{
    container::Container,
    prelude::*,
    input::{Stream,ValueInput},
    error::Rich,
    extra::Err as CE,
    Parser as CP,
};


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
                Token::Identifier(x) => TreeNode::new_term_s(SyntaxKind::NAME, x),
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
                Token::Identifier(x) => TreeNode::new_term_s(SyntaxKind::NAME, x),
    };
    let column_type = select!{Token::Identifier(type_name) => TreeNode::new_term_s(SyntaxKind::TYPE, type_name),
    };
    
    let column_constraints = set((
        just(Token::Null)
            .to(TreeNode::new_term(SyntaxKind::NULL, "NULL"))
            .or(just(Token::Not)
                    .then(just(Token::Null)).to(TreeNode::new_term(SyntaxKind::NOT_NULL, "NOT NULL"))
        ).or_not(),
        just(Token::Primary)
            .then(just(Token::Key))
            .to(TreeNode::new_term(SyntaxKind::PRIMARY_KEY, "PRYMARY KEY"))
            .or_not(),
        just(Token::Unique)
        .then(just(Token::Nulls)
                .then(just(Token::Not)
                    .or_not())
                .then(just(Token::Distinct))
                .or_not())
        .map(|(_, n)| {
            let mut node = TreeNode::new_no_term(SyntaxKind::UNIQUE, vec![]);
            if let Some(((_,no), di)) = n {
                node.push(TreeNode::new_term(SyntaxKind::NULLS, "NULLS"));
                if let (Some(Token::Not), Token::Distinct) = (no, di) {
                    node.push(TreeNode::new_term(SyntaxKind::NOT, "NOT" ));
                    node.push(TreeNode::new_term(SyntaxKind::DISTINCT, "DISTINCT"));
                }
                else {
                    node.push(TreeNode::new_term(SyntaxKind::DISTINCT, "DISTINCT"));
                }
            }
            node
        }).or_not(),
        col_foregein_key().or_not(),
    ))
    .map(|(n,pk,u,fk)|{
        let mut base = TreeNode::new_no_term(SyntaxKind::COLUMN_CONSTRAINTS, vec![]);
        for p in boxed_vec![n,pk,u,fk] {
            if let Some(x) = *p{
               base.push(x);
            }
        }
        base
    });
    
    column_name
        .then(column_type)
        .then(column_constraints)
        .map(|((n,t), c)| TreeNode::new_no_term(SyntaxKind::COLUMN, vec![Box::new(n), Box::new(t), Box::new(c)]))
}

fn col_foregein_key<'src,I>() -> impl CP<'src, I ,TreeNode, CE<Rich<'src, Token>>>
where
    I: ValueInput<'src, Token = Token, Span = SimpleSpan>
{
    let core = just(Token::References).ignore_then(
        select!{ Token::Identifier(x) => TreeNode::new_term_s(SyntaxKind::NAME, x),
        }).then(
            select!{ Token::Identifier(y) => TreeNode::new_term_s(SyntaxKind::NAME, y),
            }.delimited_by(just(Token::ParenthesesStart), just(Token::ParenthesesEnd)).or_not()
        ).map(|(tn,pn)|{
            let mut node = TreeNode::new_no_term(SyntaxKind::FOREGEIN_KEY, vec![Box::new(tn)]);
                if let Some(tc) = pn {
                    node.push(tc);
                }
                node
            });

    let r#match = just(Token::Match).ignore_then(
        choice((
            just(Token::Full).to(TreeNode::new_term(SyntaxKind::MATCH_FULL, "MATCH FULL")),
            just(Token::Partial).to(TreeNode::new_term(SyntaxKind::MATCH_PARTIAL, "MATCH PARTIAL")),
            just(Token::Simple).to(TreeNode::new_term(SyntaxKind::MATCH_SIMPLE, "MATCH SIMPLE")),
        ))
    );
    
    let cascade = just(Token::Cascade)
        .to(TreeNode::new_term(SyntaxKind::CASCADE, "CASCADE"));
    
    let restrict = just(Token::Restrict)
        .to(TreeNode::new_term(SyntaxKind::CASCADE, "RESTRICT"));
        
    let no_action = just(Token::No)
        .ignore_then(just(Token::Action))
        .to(TreeNode::new_term(SyntaxKind::NO_ACTION, "NO ACTION"));
        
    let set_default = just(Token::Set)
        .ignore_then(just(Token::Default))
        .to(TreeNode::new_term(SyntaxKind::SET_DEFAULT, "SET DEFAULT"));
        
    let set_null = just(Token::Set)
        .ignore_then(just(Token::Null))
        .to(TreeNode::new_term(SyntaxKind::SET_NULL, "SET NULL"));
    
    let on_delete = just(Token::On)
        .ignore_then(just(Token::Delete))
        .ignore_then(choice((
            cascade.clone(),
            restrict.clone(),
            set_null.clone(),
            set_default.clone(),
            no_action.clone(),
        )))
        .map(|a|
            TreeNode::new_no_term(SyntaxKind::ON_DELETE, vec![Box::new(a)])
        );
    

    let on_update = just(Token::On)
        .ignore_then(just(Token::Update))
        .ignore_then(choice((
            cascade.clone(),
            restrict.clone(),
            no_action.clone(),
        )))
        .map(|a|
            TreeNode::new_no_term(SyntaxKind::ON_UPDATE, vec![Box::new(a)])
        );
    core
        .then(r#match
                .or_not())
        .then(on_delete
                .or(on_update)
                .or_not())
        .map(|((mut c, m), oa)|{
            if let Some(x) = m {
                c.push(x);
            }
            if let Some(y) = oa {
                c.push(y);
            }
            c
        })
}

fn table_constraint<'src,I>() -> impl CP<'src, I ,TreeNode, CE<Rich<'src, Token>>>
where
    I: ValueInput<'src, Token = Token, Span = SimpleSpan>
{
    todo()
}

fn innerexp_parser<'src,I>() -> impl CP<'src, I ,TreeNode, CE<Rich<'src, Token>>>
where
    I: ValueInput<'src, Token = Token, Span = SimpleSpan>
{
    todo()
}

fn boolean<'src,I>() -> impl CP<'src, I ,TreeNode, CE<Rich<'src, Token>>>
where
    I: ValueInput<'src, Token = Token, Span = SimpleSpan>
{
    let r#true = just(Token::True).to(TreeNode::new_term(SyntaxKind::TRUE, "TRUE"));
    let r#false = just(Token::False).to(TreeNode::new_term(SyntaxKind::FALSE, "FALSE"));
     
    r#true
        .or(r#false)
}

fn identifier<'src,I>() -> impl CP<'src, I ,TreeNode, CE<Rich<'src, Token>>>
where
    I: ValueInput<'src, Token = Token, Span = SimpleSpan>
{
    select!{
        Token::Identifier(x) => TreeNode::new_term_s(SyntaxKind::IDENTIFIER, x),
    }
}

fn number<'src,I>() -> impl CP<'src, I ,TreeNode, CE<Rich<'src, Token>>>
where
    I: ValueInput<'src, Token = Token, Span = SimpleSpan>
{
    select!{
        Token::Number(x) => TreeNode::new_term_s(SyntaxKind::NUMBER, x),
    }
}

fn string<'src,I>() -> impl CP<'src, I ,TreeNode, CE<Rich<'src, Token>>>
where
    I: ValueInput<'src, Token = Token, Span = SimpleSpan>
{
    select!{
        Token::Text(x) => TreeNode::new_term_s(SyntaxKind::STRING, x),
    }
}

/*fn test<'src, I>() -> LinkedList<Rc<RefCell<Box<dyn CP<'src, I, TreeNode> + 'static >>>>
where
    I: ValueInput<'src, Token = Token, Span = SimpleSpan>
{
    parser_list!(string(), boolean(), number())
}
*/
