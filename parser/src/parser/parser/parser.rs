use crate::parser::grammar::dialect::Dialect;
use crate::parser::ast::ast_trait::Ast;
use chumsky::extra::Err as CE;
use chumsky::error::Rich;
use logos::Logos;



trait Parser<'src, A, D, Token>
where
  D: Dialect,
  A: Ast<D>,
  Token: Logos<'src> + Clone
{
 
  fn parse(&self) -> Result<A, CE<Rich<'src, Token>>>;
  fn new(source: &'src str) -> Self;
}

pub fn parse<'src, P, T, D, A>(text: &'src str) -> Result<A, CE<Rich<'src, T>>>
where 
  D: Dialect,
  T: Logos<'src> + Clone,
  A: Ast<D>,
  P: Parser<'src , A , D, T> + 'src
{
  let parser = P::new(text);
  parser.parse()
}