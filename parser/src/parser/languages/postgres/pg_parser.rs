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
    just(Token::Create)
        .ignore_then(just(Token::Table))
        .to(TreeNode::new_no_term(SyntaxKind::CREATE,
            vec![Box::new(TreeNode::new_no_term(SyntaxKind::TABLE, vec![]))]))
}

/*fn column<'src,I>() -> impl CP<'src, I ,TreeNode, CE<Rich<'src, Token>>>
where
    I: ValueInput<'src, Token = Token, Span = SimpleSpan>
{
    let name = select!{
                Token::Identifier(x) => TreeNode::new_term(SyntaxKind::NAME, x),
    };
   // let column_type = select!{Token::Identifier(type_name) => TreeNode::new_term(SyntaxKind::TYPE, type_name)};
    todo()
}*/