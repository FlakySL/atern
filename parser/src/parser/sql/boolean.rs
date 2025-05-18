use crate::parser::ast::SyntaxKind::*;
use crate::parser::grammar::Grammar::{self, Combo, Template};
use crate::parser::grammar::GrammarType::*;
use crate::parser::grammar::TemplateConfig;

pub const CMP_GRAMMAR: Grammar = Combo(
    true,
    &[
        Template(
            &[Multi(&[NUMBER, IDENTIFIER, TEXT]), Type(EQUAL), Multi(&[NUMBER, IDENTIFIER, TEXT])],
            TemplateConfig { father: COMPARE, ignore: &[EQUAL] },
        ),
        Template(
            &[Multi(&[NUMBER, IDENTIFIER]), Type(GT), Multi(&[NUMBER, IDENTIFIER])],
            TemplateConfig { father: GREATER, ignore: &[GT] },
        ),
        Template(
            &[Multi(&[NUMBER, IDENTIFIER]), Type(LT), Multi(&[NUMBER, IDENTIFIER])],
            TemplateConfig { father: LESS, ignore: &[LT] },
        ),
    ],
);
