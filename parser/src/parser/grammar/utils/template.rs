use crate::parser::ast::SyntaxKind::{self, *};
use crate::parser::ast::{AstError, Parser};
use crate::parser::grammar::GrammarType;

#[derive(Debug)]
pub struct TemplateConfig {
    pub father: SyntaxKind,
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
