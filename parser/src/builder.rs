use logos::Logos;

use super::parser::ast::{AstError, Parser, SyntaxNode};
use super::parser::lexer::Token;

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
