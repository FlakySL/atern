use crate::parser::{ast::{AstError, Parser, SyntaxKind::{self, *}}, grammar::{process::process_rule, Grammar, GrammarType}};

pub fn process_children(
    start: &GrammarType,
    body: &[Grammar],
    father: SyntaxKind,
    parser: &mut Parser,
) -> Result<(), AstError> {
    if start != &parser.peek().unwrap_or(EMPTY) {
        return Err(AstError::UnexpectedNode(parser.peek().unwrap_or(EMPTY)));
    }

    let s = parser.peek().unwrap();
    parser.builder.start_node_at(parser.builder.checkpoint(), s.into());
    parser.next();

    for rule in body {
        process_rule(&rule, father, parser)?;
    }

    parser.builder.finish_node();

    Ok(())
}
