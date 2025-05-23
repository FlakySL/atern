use crate::parser::ast::SyntaxKind::{self, *};
use crate::parser::ast::{Parser, ParserError};
use crate::parser::grammar::GrammarType;

#[derive(Debug)]
/// Configuration for the Template grammar rule
pub struct TemplateConfig {
    /// It is the data type that will wrap everything in the template.
    pub father: SyntaxKind,
    /// Indicates a node to be ignored (almost always a derivative of the parent
    /// node).
    pub ignore: SyntaxKind,
}

pub fn process_template(
    template: &[GrammarType],
    config: &TemplateConfig,
    parser: &mut Parser,
) -> Result<(), ParserError> {
    let tokens = {
        let mut tokens = Vec::new();
        for rule in template {
            if parser.peek() == Some(config.ignore) {
                parser.next();
                continue;
            }

            if *rule == parser.peek().unwrap_or(EMPTY) {
                let peek = parser.peek_with_content().unwrap();
                tokens.push((peek.0, peek.1.clone()));
                parser.next();
                continue;
            }

            return Err(ParserError::UnexpectedNode(parser.peek().unwrap_or(EMPTY)));
        }
        tokens
    };

    {
        parser.builder.start_node_at(parser.builder.checkpoint(), config.father.into());

        for t in tokens.iter() {
            parser.builder.token(t.0.into(), &t.1);
        }

        parser.builder.finish_node();
    }

    Ok(())
}
