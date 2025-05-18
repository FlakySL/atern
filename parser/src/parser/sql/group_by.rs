use crate::parser::ast::SyntaxKind::*;
use crate::parser::grammar::Grammar::{self, Template};
use crate::parser::grammar::GrammarType::*;
use crate::parser::grammar::TemplateConfig;

pub const GROUP_BY_GRAMMAR: Grammar =
    Template(&[Type(GROUP), Type(BY), List(IDENTIFIER)], TemplateConfig { father: GROUP_BY, ignore: &[GROUP, BY] });
