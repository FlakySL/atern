use crate::parser::ast::SyntaxKind::*;
use crate::parser::ast::{AstError, Parser};

pub fn process_select(parser: &mut Parser) -> Result<(), AstError> {
    parser.builder.start_node_at(parser.builder.checkpoint(), SELECT.into());
    parser.next();
    
    if parser.peek() == Some(SEMICOLON) {
        return Err(AstError::ExpectedBodyFor(SELECT));
    }

    while let Some(p) = parser.peek() {
        if p.is_special() {
            break;
        }
        parser.bump();
    }

    let val = parser.peek().unwrap().into();
    parser.builder.start_node_at(parser.builder.checkpoint(), val);
    parser.next();

    while let Some(token) = parser.peek() {
        match token {
            COMMA => {
                parser.next();

                if parser.peek() != Some(IDENTIFIER) {
                    return Err(AstError::TrailingComma);
                }

                continue;
            },
            SEMICOLON => {
                parser.builder.finish_node();
                parser.builder.finish_node();
                parser.next();
                return Ok(());
            },
            IDENTIFIER => {
                parser.bump();
            },
            _ => {
                return Err(AstError::ExpectedType(IDENTIFIER, token));
            },
        }
    }

    parser.builder.finish_node();

    parser.builder.finish_node();
    Ok(())
}
