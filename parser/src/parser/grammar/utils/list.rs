use crate::parser::ast::SyntaxKind::{self, *};
use crate::parser::ast::{Parser, ParserError};

pub fn process_list(
    t: &[SyntaxKind],
    father: SyntaxKind,
    parser: &mut Parser,
) -> Result<(), ParserError> {
    if parser.peek() == None || parser.peek() == Some(SEMICOLON) {
        return Err(ParserError::ExpectedBodyFor(father));
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
