use crate::parser::ast::SyntaxKind::*;
use crate::parser::grammar::Grammar::{self, Template};
use crate::parser::grammar::GrammarType::*;
use crate::parser::grammar::TemplateConfig;

pub const CMP_GRAMMAR: Grammar = Template(
    &[Multi(&[NUMBER, IDENTIFIER]), Type(EQUAL), Multi(&[NUMBER, IDENTIFIER])],
    TemplateConfig { father: COMPARE, ignore: EQUAL },
);
