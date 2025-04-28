use logos::Logos;

use super::parser::ast::{AstError, Parser, SyntaxNode};
use super::parser::lexer::Token;

/// This structure is an abstraction of using the lexer and passing that value
/// to the parser. Receives a String and returns a node of the ast, example:
/// SqlBuilder::from("SELECT * FROM TABLE_A WHERE age =
/// 18".to_string()).build()?
pub struct SqlBuilder {
    code: String,
}

impl SqlBuilder {
    pub fn build(self) -> Result<SyntaxNode, AstError> {
        Ok(Parser::from_tokens(&mut Token::lexer(&self.code))?.parse()?)
    }
}

impl From<String> for SqlBuilder {
    fn from(code: String) -> Self {
        Self { code }
    }
}
