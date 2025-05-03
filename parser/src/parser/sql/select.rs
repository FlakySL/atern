use super::boolean::CMP_GRAMMAR;
use crate::parser::ast::SyntaxKind::*;
use crate::parser::grammar::Grammar::{self, *};
use crate::parser::grammar::GrammarType::*;

pub const SELECT_GRAMMAR: &[Grammar] = &[
    List(&[IDENTIFIER, ALL]),
    Loop(
        &Combo(
            true,
            &[Children(Type(FROM), &[List(&[IDENTIFIER])]), Children(Type(WHERE), &[CMP_GRAMMAR])],
        ),
        SEMICOLON,
    ),
];
