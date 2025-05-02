use crate::parser::ast::SyntaxKind::{self, *};
use crate::parser::ast::{Parser, ParserError};

pub fn process_expect(
    token: SyntaxKind,
    consume: bool,
    parser: &mut Parser,
) -> Result<(), ParserError> {
    if parser.peek() != Some(token) {
        return Err(ParserError::ExpectedType(token, parser.peek().unwrap_or(EMPTY)));
    }

    if consume {
        parser.bump();
    } else {
        parser.next();
    }

    Ok(())
}
