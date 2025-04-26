use crate::parser::ast::SyntaxKind::{self, *};
use crate::parser::ast::{AstError, Parser};

pub fn process_expect(
    token: SyntaxKind,
    consume: bool,
    parser: &mut Parser,
) -> Result<(), AstError> {
    if parser.peek() != Some(token) {
        return Err(AstError::ExpectedType(token, parser.peek().unwrap_or(EMPTY)));
    }

    if consume {
        parser.bump();
    } else {
        parser.next();
    }

    Ok(())
}
