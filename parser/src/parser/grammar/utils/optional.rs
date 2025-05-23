use crate::parser::ast::{Parser, SyntaxKind};
use crate::parser::grammar::process::process_rule;
use crate::parser::grammar::Grammar;

pub fn process_optional(rule: &Grammar, father: SyntaxKind, parser: &mut Parser) {
    let iter = parser.iter.clone();

    process_rule(rule, father, parser).unwrap_or_else(|_| {
        parser.iter = iter;
    });
}
