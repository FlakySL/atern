use crate::parser::{ast::{AstError, Parser, SyntaxKind}, grammar::{process::process_rule, Grammar}};

pub fn process_combo(
    optional: bool,
    children: &[Grammar],
    father: SyntaxKind,
    parser: &mut Parser,
) -> Result<(), AstError> {
    let mut good = true;

    for child in children {
        match process_rule(&child, father, parser) {
            Ok(_) => {
                good = true;
                break;
            },
            Err(_) => {
                good = false;
                break;
            },
        };
    }

    if !good && !optional {
        return Err(AstError::ExpectedBodyFor(father));
    }

    if !good {
        parser.next();
    }

    Ok(())
}

