///! Shortcut to parse sql code
use logos::Logos;

use super::parser::ast::{ParserError, Parser, SyntaxNode};
use super::parser::lexer::Token;

/// This structure is an abstraction of using the lexer and passing that value
/// to the parser. Receives a String and returns a node of the ast
pub struct SqlBuilder {
    code: String,
}

impl SqlBuilder {
    pub fn build(self) -> Result<SyntaxNode, ParserError> {
        Ok(Parser::from_tokens(&mut Token::lexer(&self.code))?.parse()?)
    }
}

impl From<String> for SqlBuilder {
    fn from(code: String) -> Self {
        Self { code }
    }
}
