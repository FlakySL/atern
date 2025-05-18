use super::list::process_list;
use crate::parser::ast::SyntaxKind::{self, *};
use crate::parser::ast::{Parser, ParserError};
use crate::parser::grammar::GrammarType::{self, *};

#[derive(Debug)]
/// Configuration for the Template grammar rule
pub struct TemplateConfig {
    /// It is the data type that will wrap everything in the template.
    pub father: SyntaxKind,
    /// Indicates a node to be ignored (almost always a derivative of the parent
    /// node).
    pub ignore: &'static [SyntaxKind],
}

pub fn process_template(
    template: &[GrammarType],
    config: &TemplateConfig,
    parser: &mut Parser,
) -> Result<(), ParserError> {
    parser.builder.start_node_at(parser.builder.checkpoint(), config.father.into());
    let tokens = {
        let mut tokens = Vec::new();
        for rule in template {
            if config.ignore.contains(&parser.peek().unwrap_or(EMPTY)) {
                parser.next();
                continue;
            }
            if let List(t) = rule {
                process_list(&[*t], parser)?;
                if parser.peek().is_none() {
                    parser.builder.finish_node();

                    return Ok(());
                }
                continue;
            } else if *rule == parser.peek().unwrap_or(EMPTY) {
                let peek = parser.peek_with_content().unwrap();
                if !config.ignore.contains(&peek.0) {
                    tokens.push((peek.0, peek.1.clone()));
                }
                parser.next();
            } else {
                return Err(ParserError::UnexpectedNode(parser.peek().unwrap_or(EMPTY)));
            }
        }
        tokens
    };

    for t in tokens.iter() {
        parser.builder.token(t.0.into(), &t.1);
    }
    parser.builder.finish_node();

    Ok(())
}

