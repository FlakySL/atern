use crate::parser::ast::SyntaxKind::*;
use crate::parser::ast::{AstError, Parser};

pub fn process_create(parser: &mut Parser) -> Result<(), AstError> {
    parser.builder.start_node_at(parser.builder.checkpoint(), CREATE.into());
    parser.next();

    match parser.peek() {
        Some(t) => {
            if !t.is_ddl() {
                return Err(AstError::UnexpectedNode(t));
            }

            parser.bump();
        },
        None => {
            return Err(AstError::ExpectedBodyFor(CREATE));
        },
    }

    match parser.peek() {
        Some(t) => {
            if t == IDENTIFIER {
                parser.bump();
            } else {
                return Err(AstError::ExpectedType(IDENTIFIER, t));
            }
        },
        None => {
            return Err(AstError::ExpectedBodyFor(CREATE));
        },
    }

    parser.builder.finish_node();
    Ok(())
}
