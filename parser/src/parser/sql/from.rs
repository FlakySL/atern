use crate::parser::ast::SyntaxKind::*;
use crate::parser::grammar::Grammar::{self, *};
use crate::parser::grammar::GrammarType::Type;

pub const FROM_GRAMMAR: Grammar = Children(Type(FROM), &[List(&[IDENTIFIER])]);
