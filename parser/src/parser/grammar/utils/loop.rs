use crate::parser::{ast::{AstError, Parser, SyntaxKind}, grammar::{process::process_rule, Grammar}};

pub fn process_loop(
    child: &Box<Grammar>,
    stop: SyntaxKind,
    father: SyntaxKind,
    parser: &mut Parser,
) -> Result<(), AstError> {
    while parser.peek() != Some(stop) && parser.peek() != None {
        process_rule(&child, father, parser)?;
    }

    Ok(())
}


