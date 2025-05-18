use super::boolean::CMP_GRAMMAR;
use crate::parser::ast::SyntaxKind::*;
use crate::parser::grammar::Grammar::{self, Children};
use crate::parser::grammar::GrammarType::Type;

pub const WHERE_GRAMMAR: Grammar = Children(Type(WHERE), &[CMP_GRAMMAR]);
