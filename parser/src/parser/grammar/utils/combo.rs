use crate::parser::ast::{Parser, ParserError, SyntaxKind};
use crate::parser::grammar::process::process_rule;
use crate::parser::grammar::Grammar;

pub fn process_combo(
    optional: bool,
    children: &[Grammar],
    father: SyntaxKind,
    parser: &mut Parser,
) -> Result<(), ParserError> {
    let mut good = true;

    for child in children {
        match process_rule(&child, father, parser) {
            Ok(_) => {
                good = true;
                break;
            },
            Err(_) => {
                good = false;
            },
        };
    }

    if !good && !optional {
        return Err(ParserError::ExpectedBodyFor(father));
    }

    if !good {
        parser.next();
    }

    Ok(())
}
