use crate::parser::ast::{Parser, ParserError, SyntaxKind};
use crate::parser::grammar::process::process_rule;
use crate::parser::grammar::Grammar;

pub fn process_loop(
    child: &Grammar,
    stop: SyntaxKind,
    father: SyntaxKind,
    parser: &mut Parser,
) -> Result<(), ParserError> {
    while parser.peek() != Some(stop) && parser.peek() != None {
        process_rule(&child, father, parser)?;
    }

    Ok(())
}
