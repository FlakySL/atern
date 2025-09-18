
use super::pg_kind::PgKind;
use crate::parser::parser::parser::Parser as PP;
use super::pg_lexer::PgToken;
use crate::boxed_vec;
use logos::Logos;
use super::pgsql::PostgreSQL;
use crate::parser::parser::utils::some_of;
use crate::parser::ast::nodes::TreeNode;
use chumsky::{
    container::Container,
    prelude::*,
    pratt::*,
    input::{Stream,ValueInput},
    error::Rich,
    extra::Err as CE,
    Parser as CP,
};


pub struct PgParser<'src>{
  source: &'src str,
}

impl <'src> PP<'src, PgKind, PostgreSQL, PgToken> for PgParser<'src>{
    fn new(source: &'src str) -> Self{
        PgParser{
            source
        }
    }
    
    fn parse(&mut self) -> ParseResult<TreeNode<PgKind>, Rich<'src,PgToken>>{
        let token_iter = PgToken::lexer(self.source)
                        .spanned()
                        .map(|(tok, span)| match tok {
                            Ok(tok) => (tok, span.into()),
                            Err(()) => (PgToken::Error, span.into()),
                        });

    let token_stream = Stream::from_iter(token_iter)
                        .map((0..self.source.len()).into(), |(t, s)| (t, s));

        start().parse(token_stream)
    }
}
//=====================PARSERS=================//
fn start<'src, I>() -> impl CP<'src, I ,TreeNode<PgKind>, CE<Rich<'src,PgToken>>>
where
    I: ValueInput<'src, Token = PgToken, Span = SimpleSpan>
{
    create_table()
}

fn create_table<'src,I>() -> impl CP<'src, I ,TreeNode<PgKind>, CE<Rich<'src, PgToken>>>
where
    I: ValueInput<'src, Token = PgToken, Span = SimpleSpan>
{
    let create = just(PgToken::Create).to(TreeNode::new_no_term(PgKind::CREATE,vec![]));
    let table = just(PgToken::Table).to(TreeNode::new_no_term(PgKind::TABLE, vec![]));
    let table_name = select!{
                PgToken::Identifier(x) => TreeNode::new_term_s(PgKind::NAME, x),
    };
    create
        .then(table)
        .then(table_name)
        .then(
            column()
                .map(Box::new)
                .separated_by(just(PgToken::Comma))
                .collect::<Vec<_>>()
                .delimited_by(
                    just(PgToken::ParenthesesStart),
                    just(PgToken::ParenthesesEnd)
                        .ignore_then(just(PgToken::Semicolon))
                )
                
        )
        .map(|(((mut c,mut t),tn),cols)| {
            t.push(tn);
            t.push(TreeNode::new_no_term(PgKind::TABLE_BODY, cols));
            c.push(t);
            c
        })
}

fn column<'src,I>() -> impl CP<'src, I ,TreeNode<PgKind>, CE<Rich<'src, PgToken>>>
where
    I: ValueInput<'src, Token = PgToken, Span = SimpleSpan>
{
    let column_name = select!{
                PgToken::Identifier(x) => TreeNode::new_term_s(PgKind::NAME, x),
    };
    let column_type = select!{PgToken::Identifier(type_name) => TreeNode::new_term_s(PgKind::TYPE, type_name),
    };
    
    let column_constraints = some_of((
        just(PgToken::Null)
            .to(TreeNode::new_term(PgKind::NULL, "NULL"))
            .or(just(PgToken::Not)
                    .then(just(PgToken::Null)).to(TreeNode::new_term(PgKind::NOT_NULL, "NOT NULL"))
        ),
        just(PgToken::Primary)
            .then(just(PgToken::Key))
            .to(TreeNode::new_term(PgKind::PRIMARY_KEY, "PRYMARY KEY")),
        just(PgToken::Unique)
        .then(just(PgToken::Nulls)
                .then(just(PgToken::Not)
                    .or_not())
                .then(just(PgToken::Distinct))
                .or_not())
        .map(|(_, n)| {
            let mut node = TreeNode::new_no_term(PgKind::UNIQUE, vec![]);
            if let Some(((_,no), di)) = n {
                node.push(TreeNode::new_term(PgKind::NULLS, "NULLS"));
                if let (Some(PgToken::Not), PgToken::Distinct) = (no, di) {
                    node.push(TreeNode::new_term(PgKind::NOT, "NOT" ));
                    node.push(TreeNode::new_term(PgKind::DISTINCT, "DISTINCT"));
                }
                else {
                    node.push(TreeNode::new_term(PgKind::DISTINCT, "DISTINCT"));
                }
            }
            node
        }),
        just(PgToken::Check)
            .ignore_then(inner_expr())
            .map(|x| TreeNode::new_no_term(PgKind::CHECK, boxed_vec![x])),
        col_foregein_key(),
    ))
    .map(|v|
        TreeNode::new_no_term(PgKind::COLUMN_CONSTRAINTS, v));
    
    column_name
        .then(column_type)
        .then(column_constraints)
        .map(|((n,t), c)| TreeNode::new_no_term(PgKind::COLUMN, vec![Box::new(n), Box::new(t), Box::new(c)]))
}

fn col_foregein_key<'src,I>() -> impl CP<'src, I ,TreeNode<PgKind>, CE<Rich<'src, PgToken>>>
where
    I: ValueInput<'src, Token = PgToken, Span = SimpleSpan>
{
    let core = just(PgToken::References).ignore_then(
        select!{ PgToken::Identifier(x) => TreeNode::new_term_s(PgKind::NAME, x),
        }).then(
            select!{ PgToken::Identifier(y) => TreeNode::new_term_s(PgKind::NAME, y),
            }.delimited_by(just(PgToken::ParenthesesStart), just(PgToken::ParenthesesEnd)).or_not()
        ).map(|(tn,pn)|{
            let mut node = TreeNode::new_no_term(PgKind::FOREGEIN_KEY, vec![Box::new(tn)]);
                if let Some(tc) = pn {
                    node.push(tc);
                }
                node
            });

    let r#match = just(PgToken::Match).ignore_then(
        choice((
            just(PgToken::Full).to(TreeNode::new_term(PgKind::MATCH_FULL, "MATCH FULL")),
            just(PgToken::Partial).to(TreeNode::new_term(PgKind::MATCH_PARTIAL, "MATCH PARTIAL")),
            just(PgToken::Simple).to(TreeNode::new_term(PgKind::MATCH_SIMPLE, "MATCH SIMPLE")),
        ))
    );
    
    let cascade = just(PgToken::Cascade)
        .to(TreeNode::new_term(PgKind::CASCADE, "CASCADE"));
    
    let restrict = just(PgToken::Restrict)
        .to(TreeNode::new_term(PgKind::CASCADE, "RESTRICT"));
        
    let no_action = just(PgToken::No)
        .ignore_then(just(PgToken::Action))
        .to(TreeNode::new_term(PgKind::NO_ACTION, "NO ACTION"));
        
    let set_default = just(PgToken::Set)
        .ignore_then(just(PgToken::Default))
        .to(TreeNode::new_term(PgKind::SET_DEFAULT, "SET DEFAULT"));
        
    let set_null = just(PgToken::Set)
        .ignore_then(just(PgToken::Null))
        .to(TreeNode::new_term(PgKind::SET_NULL, "SET NULL"));
    
    let on_delete = just(PgToken::On)
        .ignore_then(just(PgToken::Delete))
        .ignore_then(choice((
            cascade.clone(),
            restrict.clone(),
            set_null.clone(),
            set_default.clone(),
            no_action.clone(),
        )))
        .map(|a|
            TreeNode::new_no_term(PgKind::ON_DELETE, vec![Box::new(a)])
        );
    

    let on_update = just(PgToken::On)
        .ignore_then(just(PgToken::Update))
        .ignore_then(choice((
            cascade.clone(),
            restrict.clone(),
            no_action.clone(),
        )))
        .map(|a|
            TreeNode::new_no_term(PgKind::ON_UPDATE, vec![Box::new(a)])
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

fn table_constraint<'src,I>() -> impl CP<'src, I ,TreeNode<PgKind>, CE<Rich<'src,PgToken>>>
where
    I: ValueInput<'src, Token =PgToken, Span = SimpleSpan>
{
    todo()
}

fn inner_expr<'src,I>() -> impl CP<'src, I ,TreeNode<PgKind>, CE<Rich<'src,PgToken>>>
where
    I: ValueInput<'src, Token = PgToken, Span = SimpleSpan>
{   recursive(|expr|{
        let operand = choice((
            number(),
            boolean(),
            string(),
            identifier(),
            expr.clone()
        ));
        let op = |o| just(o);
        operand.pratt((
            infix(left(1), op(PgToken::Or), |l, _, r, _|{
                TreeNode::new_no_term(PgKind::OR, boxed_vec![l,r])
            }),
            infix(left(2), op(PgToken::And), |l, _, r, _|{
                TreeNode::new_no_term(PgKind::AND, boxed_vec![l,r])
            }),
            prefix(3, op(PgToken::Not), |_, r, _|{
                TreeNode::new_no_term(PgKind::OR, boxed_vec![r])
            }),
            infix(none(5), op(PgToken::Equal), |l, _, r, _|{
                TreeNode::new_no_term(PgKind::EQUAL, boxed_vec![l,r])
            }),
            infix(none(5), op(PgToken::UnEqual), |l, _, r, _|{
                TreeNode::new_no_term(PgKind::NOT_EQUAL, boxed_vec![l,r])
            }),
            infix(none(5), op(PgToken::GT), |l, _, r, _|{
                TreeNode::new_no_term(PgKind::GT, boxed_vec![l,r])
            }),
            infix(none(5), op(PgToken::LT), |l, _, r, _|{
                TreeNode::new_no_term(PgKind::LT, boxed_vec![l,r])
            }),
            infix(none(5), op(PgToken::GEQT), |l, _, r, _|{
                TreeNode::new_no_term(PgKind::GEQT, boxed_vec![l,r])
            }),
            infix(none(5), op(PgToken::LEQT), |l, _, r, _|{
                TreeNode::new_no_term(PgKind::LEQT, boxed_vec![l,r])
            }),
            infix(left(8), op(PgToken::Plus), |l, _, r, _|{
                TreeNode::new_no_term(PgKind::ADD, boxed_vec![l,r])
            }),
            infix(left(8), op(PgToken::Minus), |l, _, r, _|{
                TreeNode::new_no_term(PgKind::SUB, boxed_vec![l,r])
            }),
            infix(left(9), op(PgToken::Star), |l, _, r, _|{
                TreeNode::new_no_term(PgKind::MUL, boxed_vec![l,r])
            }),
            infix(left(9), op(PgToken::Slash), |l, _, r, _|{
                TreeNode::new_no_term(PgKind::DIV, boxed_vec![l,r])
            }),
            infix(left(9), op(PgToken::Mod), |l, _, r, _|{
                TreeNode::new_no_term(PgKind::MODULO, boxed_vec![l,r])
            }),
            infix(left(10), op(PgToken::Exp), |l, _, r, _|{
                TreeNode::new_no_term(PgKind::EXPONENTIAL, boxed_vec![l,r])
            }),
            prefix(13, op(PgToken::Minus), |_, r, _|{
                TreeNode::new_no_term(PgKind::NEG, boxed_vec![r])
            }),
            prefix(13, op(PgToken::Plus), |_, r, _|{
                TreeNode::new_no_term(PgKind::POS, boxed_vec![r])
            }),
            infix(left(16), op(PgToken::Dot), |l, _, r, _|{
                TreeNode::new_no_term(PgKind::TABLE_COL, boxed_vec![l,r])
            }),
        ))
    })
    .delimited_by(just(PgToken::ParenthesesStart), just(PgToken::ParenthesesEnd))
    .map(|all|
            TreeNode::new_no_term(PgKind::EXPRESSION, boxed_vec![all])
        )
}

fn boolean<'src,I>() -> impl CP<'src, I ,TreeNode<PgKind>, CE<Rich<'src,PgToken>>> + Clone
where
    I: ValueInput<'src, Token= PgToken, Span = SimpleSpan>
{
    let r#true = just(PgToken::True).to(TreeNode::new_term(PgKind::TRUE, "TRUE"));
    let r#false = just(PgToken::False).to(TreeNode::new_term(PgKind::FALSE, "FALSE"));
     
    r#true
        .or(r#false)
}

fn identifier<'src,I>() -> impl CP<'src, I ,TreeNode<PgKind>, CE<Rich<'src,PgToken>>> + Clone
where
    I: ValueInput<'src, Token= PgToken, Span = SimpleSpan> 
{
    select!{
        PgToken::Identifier(x) => TreeNode::new_term_s(PgKind::IDENTIFIER, x),
    }
}

fn number<'src,I>() -> impl CP<'src, I ,TreeNode<PgKind>, CE<Rich<'src,PgToken>>> + Clone
where
    I: ValueInput<'src, Token= PgToken, Span = SimpleSpan>
{
    select!{
        PgToken::Number(x) => TreeNode::new_term_s(PgKind::NUMBER, x),
    }
}

fn string<'src,I>() -> impl CP<'src, I ,TreeNode<PgKind>, CE<Rich<'src,PgToken>>> + Clone 
where
    I: ValueInput<'src, Token = PgToken, Span = SimpleSpan>
{
    select!{
        PgToken::Text(x) => TreeNode::new_term_s(PgKind::STRING, x),
    }
}