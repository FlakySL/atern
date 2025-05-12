use crate::parser::grammar::{Grammar::{self, *}, GrammarType::Type};
use crate::parser::ast::SyntaxKind::*;

pub const FROM_GRAMMAR: Grammar = Children(Type(FROM), &[List(&[IDENTIFIER])]);
