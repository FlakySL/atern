use crate::parser::ast::SyntaxKind::{self, *};
use crate::parser::ast::{Parser, ParserError};

pub fn process_list(t: &[SyntaxKind], parser: &mut Parser) -> Result<(), ParserError> {
    if parser.peek() == None {
        return Err(ParserError::UnexpectedEof);
    }

    while let Some(token) = parser.peek() {
        if token == COMMA {
            parser.next();

            if !t.contains(&parser.peek().unwrap_or(EMPTY)) {
                return Err(ParserError::TrailingComma);
            }

            continue;
        }

        if !t.contains(&token) {
            break;
        }

        parser.bump();
    }

    Ok(())
}
