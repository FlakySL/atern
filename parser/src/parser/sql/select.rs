use super::from::FROM_GRAMMAR;
use super::group_by::GROUP_BY_GRAMMAR;
use super::order_by::ORDER_BY_GRAMMAR;
use super::swhere::WHERE_GRAMMAR;
use crate::parser::ast::SyntaxKind::*;
use crate::parser::grammar::Grammar::{self, *};

pub const SELECT_GRAMMAR: &[Grammar] = &[
    List(&[IDENTIFIER, ALL]),
    Loop(&Combo(true, &[FROM_GRAMMAR, WHERE_GRAMMAR, GROUP_BY_GRAMMAR, ORDER_BY_GRAMMAR]), SEMICOLON),
];
