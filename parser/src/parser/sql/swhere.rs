use crate::parser::grammar::{Grammar::{self, Children}, GrammarType::Type};
use crate::parser::ast::SyntaxKind::*;

use super::boolean::CMP_GRAMMAR;

pub const WHERE_GRAMMAR: Grammar = Children(Type(WHERE), &[CMP_GRAMMAR]);

