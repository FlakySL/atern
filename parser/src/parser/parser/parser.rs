use crate::parser::grammar::dialect::Dialect;
use crate::parser::ast::ast_trait::Ast;
use chumsky::error::Rich;
use logos::Logos;
use chumsky::prelude::ParseResult;



pub trait Parser<'src,N, D, Token>
where
  D: Dialect<N>,
  Token: Logos<'src> + Clone
{
 
  fn parse(&mut self) -> ParseResult<N, Rich<'src, Token>>;
  fn new(source: &'src str) -> Self;
}

pub fn parse<'src,N, P, T, D, A>(text: &'src str) -> Result<A, Vec<Rich<'src, T>>>
where 
  D: Dialect<N>,
  T: Logos<'src> + Clone,
  A: Ast<N>,
  P: Parser<'src, N, D, T> + 'src
{
  let mut parser = P::new(text);
  let output = parser.parse();
  match output.into_result(){
      Ok(x) => Ok(A::from_node(x)),
      Err(e) => Err(e)
  }
}