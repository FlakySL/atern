use crate::parser::{ast::{Parser, SyntaxKind}, grammar::{process::process_rule, Grammar}};

pub fn process_optional(rule: &Grammar, father: SyntaxKind, parser: &mut Parser) {
    let iter = parser.iter.clone();

    process_rule(rule, father, parser).unwrap_or_else(|_| {
        parser.iter = iter;
    });
}
