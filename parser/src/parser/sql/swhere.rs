use super::boolean::CMP_GRAMMAR;
use crate::parser::ast::SyntaxKind::*;
use crate::parser::grammar::Grammar::{self, Children};

pub const WHERE_GRAMMAR: Grammar = Children(&[WHERE], WHERE, &[CMP_GRAMMAR]);
