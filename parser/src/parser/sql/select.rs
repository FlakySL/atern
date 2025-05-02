use crate::parser::ast::SyntaxKind::*;
use crate::parser::grammar::Grammar::{self, *};
use crate::parser::grammar::GrammarType::*;
use super::boolean::CMP_GRAMMAR;

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
                        &[CMP_GRAMMAR],
                    )],
                ),
            ],
        ),
        SEMICOLON,
    ),
];
