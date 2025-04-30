use crate::parser::ast::SyntaxKind::*;
use crate::parser::grammar::Grammar::{self, *};
use crate::parser::grammar::GrammarType::*;
use crate::parser::grammar::TemplateConfig;

pub const SELECT_GRAMMAR: &[Grammar] = &[
    List(&[IDENTIFIER, ALL]),
    Loop(
        &Combo(
            true,
            &[
                Children(Type(FROM), &[List(&[IDENTIFIER])]),
                Combo(
                    true,
                    &[Children(
                        Type(WHERE),
                        &[Template(
                            &[
                                Multi(&[NUMBER, IDENTIFIER]),
                                Type(EQUAL),
                                Multi(&[NUMBER, IDENTIFIER]),
                            ],
                            TemplateConfig { father: COMPARE, ignore: EQUAL },
                        )],
                    )],
                ),
            ],
        ),
        SEMICOLON,
    ),
];
