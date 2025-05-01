use super::utils::children::process_children;
use super::utils::combo::process_combo;
use super::utils::expect::process_expect;
use super::utils::list::process_list;
use super::utils::r#loop::process_loop;
use super::utils::template::process_template;
use super::Grammar;
use crate::parser::ast::{AstError, Parser, SyntaxKind};

/// Takes an instance of the Parser, a parent node, and a list with definitions of the grammar to parse, returns nothing but modifies the parser.
pub fn process_grammar(
    parser: &mut Parser,
    father: SyntaxKind,
    grammar: &[Grammar],
) -> Result<(), AstError> {
    parser.builder.start_node_at(parser.builder.checkpoint(), father.into());
    parser.next();

    for rule in grammar {
        process_rule(&rule, father, parser)?;
    }

    parser.builder.finish_node();

    Ok(())
}

/// Takes a rule to be processed and executes the function that does the corresponding function depending on the definition of that rule.
pub fn process_rule(
    rule: &Grammar,
    father: SyntaxKind,
    parser: &mut Parser,
) -> Result<(), AstError> {
    match rule {
        Grammar::List(t) => {
            process_list(t, father, parser)?;
        },
        Grammar::Loop(child, stop) => {
            process_loop(child, *stop, father, parser)?;
        },
        Grammar::Combo(optional, children) => {
            process_combo(*optional, children, father, parser)?;
        },
        Grammar::Children(start, body) => {
            process_children(start, body, father, parser)?;
        },
        Grammar::Expect(token, consume) => {
            process_expect(*token, *consume, parser)?;
        },
        Grammar::Template(template, config) => {
            process_template(template, config, parser)?;
        },
    }

    Ok(())
}
