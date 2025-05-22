use crate::parser::ast::SyntaxKind::{self, *};
use crate::parser::ast::{Parser, ParserError};
use crate::parser::grammar::process::process_rule;
use crate::parser::grammar::Grammar;

pub fn process_children(
    start: &[SyntaxKind],
    node_father: SyntaxKind,
    body: &[Grammar],
    father: SyntaxKind,
    parser: &mut Parser,
) -> Result<(), ParserError> {
    for rule in start {
        if parser.peek() != Some(*rule) {
            return Err(ParserError::UnexpectedNode(parser.peek().unwrap_or(EMPTY)));
        }

        parser.next();
    }

    parser.builder.start_node_at(parser.builder.checkpoint(), node_father.into());

    for rule in body {
        process_rule(&rule, father, parser)?;
    }

    parser.builder.finish_node();

    Ok(())
}
