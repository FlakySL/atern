use crate::parser::ast::SyntaxKind::{self, *};
use crate::parser::ast::{AstError, Parser};
use crate::parser::grammar::GrammarType;

#[derive(Debug)]
/// Configuration for the Template grammar rule
pub struct TemplateConfig {
    /// It is the data type that will wrap everything in the template.
    pub father: SyntaxKind,
    /// Indicates a node to be ignored (almost always a derivative of the parent node).
    pub ignore: SyntaxKind,
}

pub fn process_template(
    template: &[GrammarType],
    config: &TemplateConfig,
    parser: &mut Parser,
) -> Result<(), AstError> {
    parser.builder.start_node_at(parser.builder.checkpoint(), config.father.into());

    for rule in template {
        if parser.peek() == Some(config.ignore) {
            parser.next();
            continue;
        }

        if *rule == parser.peek().unwrap_or(EMPTY) {
            parser.bump();
            continue;
        }

        return Err(AstError::UnexpectedNode(parser.peek().unwrap_or(EMPTY)));
    }

    parser.builder.finish_node();
    Ok(())
}
