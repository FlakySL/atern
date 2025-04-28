use crate::parser::ast::SyntaxKind::{self, *};
use crate::parser::ast::{AstError, Parser};

pub fn process_list(
    t: &[SyntaxKind],
    father: SyntaxKind,
    parser: &mut Parser,
) -> Result<(), AstError> {
    if parser.peek() == None || parser.peek() == Some(SEMICOLON) {
        return Err(AstError::ExpectedBodyFor(father));
    }

    while let Some(token) = parser.peek() {
        if token == COMMA {
            parser.next();

            if !t.contains(&parser.peek().unwrap_or(EMPTY)) {
                return Err(AstError::TrailingComma);
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
