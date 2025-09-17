use crate::parser::grammar::dialect::Dialect;
use crate::parser::ast::ast_trait::Ast;
use chumsky::error::Rich;
use logos::Logos;
use chumsky::prelude::ParseResult;
use crate::parser::ast::syntax_kind::SyntaxKind;
use crate::TreeNode;

pub trait Parser<'src,K, D, Token>
where
    K: SyntaxKind,
    D: Dialect<K>,
    Token: Logos<'src> + Clone
{
 
  fn parse(&mut self) -> ParseResult<TreeNode<K>, Rich<'src, Token>>;
  fn new(source: &'src str) -> Self;
}

pub fn parse<'src,K, P, T, D, A>(text: &'src str) -> Result<A, Vec<Rich<'src, T>>>
where
    K: SyntaxKind,
    D: Dialect<K>,
    T: Logos<'src> + Clone,
    A: Ast<K>,
    P: Parser<'src, K, D, T> + 'src
{
  let mut parser = P::new(text);
  let output = parser.parse();
  match output.into_result(){
      Ok(x) => Ok(A::from_node(x)),
      Err(e) => Err(e)
  }
}